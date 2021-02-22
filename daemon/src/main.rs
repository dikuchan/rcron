#![feature(map_first_last)]

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

use common::{
    create_rcron_directory, 
    get_socket_path, get_scheduler_path,
    plan::ExecutionPlan,
};

type Scheduler = BTreeMap<u64, ExecutionPlan>;

// TODO: Log.
// TODO: Tests.

fn schedule(receiver: Receiver<ExecutionPlan>) {
    let path = get_scheduler_path();

    let mut scheduler = match Scheduler::load(&path) {
        Ok(s) => s,
        Err(_) => {
            let mut scheduler = Scheduler::new();
            // A sender is infallible (terminates with the main thread).
            let initial = receiver.recv().unwrap();
            scheduler.insert(initial.time, initial);

            scheduler
        }
    };

    loop {
        let (timeout, plan) = match scheduler.pop_first() {
            Some((t, p)) => (t, p),
            None => continue,
        };
        let timeout = Duration::from_secs(timeout);

        match receiver.recv_timeout(timeout) {
            Ok(plan) => {
                scheduler.insert(plan.time, plan);
            },
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        Command::new(plan.command)
                            .args(plan.args)
                            .uid(plan.uid)
                            .gid(plan.gid)
                            .spawn()
                            .unwrap();
                    },
                    RecvTimeoutError::Disconnected => continue,
                }
            },
        }

        // Save the state after an update.
        let _ = scheduler.save(&path);
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
                Ok(_) => print!("Added plan: ok"),
                Err(e) => eprintln!("Cannot add plan: {}", e),
            }
        });
    }

    Ok(())
}

fn main() {
    match bind()
        .map_err(|e| e)
        .and_then(listen)
        .map_err(|e| e) {
        Ok(_) => println!("Daemon stopped: ok"),
        Err(e) => eprintln!("Cannot start daemon: {}", e),
    }
}
