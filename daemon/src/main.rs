#![feature(map_first_last)]
#![feature(btree_retain)]

pub mod error;
pub mod record;

use crate::{
    error::DaemonResult,
    record::Record,
};

use std::{
    collections::BTreeMap,
    fs,
    os::unix::{
        net::UnixListener,
        process::CommandExt,
    },
    process::Command,
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::Duration,
}; 

use chrono::Local;
use common::{
    create_rcron_directory, 
    get_socket_path, get_scheduler_path, get_journal_path,
    plan::ExecutionPlan,
};
use log::{info, LevelFilter};

type Scheduler = BTreeMap<u64, ExecutionPlan>;

// TODO: Tests.

fn schedule(receiver: Receiver<ExecutionPlan>) {
    let path = get_scheduler_path();

    let mut scheduler = match Scheduler::load(&path) {
        Ok(s) => { 
            info!("Loaded the saved state");
            s 
        },
        Err(_) => {
            let mut scheduler = Scheduler::new();
            // A sender is infallible (terminates with the main thread).
            let initial = receiver.recv().unwrap();
            info!("Added task, execution in {}", initial.time);
            scheduler.insert(initial.time, initial);

            scheduler
        }
    };

    loop {
        let time = match scheduler.first_entry() {
            // Key is u64.
            Some(e) => e.key().clone(),
            None => {
                let task = receiver.recv().unwrap();
                scheduler.insert(task.time, task);
                continue;
            },
        };
        let now = Local::now().timestamp();
        let timeout = Duration::from_secs(time - now as u64);
                
        scheduler.save(&path).unwrap();

        match receiver.recv_timeout(timeout) {
            Ok(task) => {
                info!("Added task, execution in {}", task.time);
                scheduler.insert(task.time, task);
            },
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        // The entry is checked.
                        let (_, task) = scheduler.pop_first().unwrap();
                        info!("Started task {:?}", task);
                        Command::new(task.command)
                            .args(task.args)
                            .uid(task.uid)
                            .gid(task.gid)
                            .spawn()
                            .unwrap();
                    },
                    RecvTimeoutError::Disconnected => continue,
                }
            },
        }
    }
}

fn bind() -> DaemonResult<UnixListener> {
    let socket_path = get_socket_path();
    
    create_rcron_directory()?;
    let _ = fs::remove_file(&socket_path);

    Ok(UnixListener::bind(&socket_path)?)
}

fn listen(listener: UnixListener) -> DaemonResult<()> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || schedule(receiver));

    for stream in listener.incoming() {
        let stream = stream?;
        let sender = sender.clone();

        thread::spawn(move || {
            match ExecutionPlan::receive(stream)
                .map_err(|e| e)
                .and_then(|plan| Ok(sender.send(plan)))
                .map_err(|e| e) {
                Ok(_) => print!("Added task: ok"),
                Err(e) => eprintln!("Cannot add task: {}", e),
            }
        });
    }

    Ok(())
}

fn main() {
    let journal_path = get_journal_path();
    let _ = simple_logging::log_to_file(journal_path, LevelFilter::Info);

    info!("Daemon started");

    match bind()
            .map_err(|e| e)
            .and_then(listen)
            .map_err(|e| e) {
        Ok(_) => println!("Daemon stopped: ok"),
        Err(e) => eprintln!("Cannot start daemon: {}", e),
    }

    info!("Daemon stopped");
}
