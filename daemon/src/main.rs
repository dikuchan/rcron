#![feature(map_first_last)]
#![feature(btree_retain)]

pub mod error;
pub mod cache;

use crate::{
    error::DaemonResult,
    cache::Cache,
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
    create_socket_dir,
    get_socket_name, get_cache_name, get_journal_name,
    job::Job,
};
use log::{info, error, LevelFilter};

type Time = u64;
type Scheduler = BTreeMap<Time, Job>;

// TODO: Tests.

fn schedule(receiver: Receiver<Job>) {
    let cache_name = get_cache_name();

    let mut scheduler = match Scheduler::load(&cache_name) {
        Ok(s) => s,
        Err(_) => {
            let mut scheduler = Scheduler::new();
            // A sender is infallible (terminates with the main thread).
            let job = receiver.recv().unwrap();
            scheduler.insert(job.time, job);
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
 
        scheduler.save(&cache_name).unwrap();

        match receiver.recv_timeout(timeout) {
            Ok(job) => {
                scheduler.insert(job.time, job);
            },
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        // The entry is checked.
                        let (_, task) = scheduler.pop_first().unwrap();
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
    let socket_name = get_socket_name();
    
    create_socket_dir()?;
    let _ = fs::remove_file(&socket_name);

    Ok(UnixListener::bind(&socket_name)?)
}

fn listen(listener: UnixListener) -> DaemonResult<()> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || schedule(receiver));

    for stream in listener.incoming() {
        let stream = stream?;
        let sender = sender.clone();
        match Job::receive(stream)
                  .map_err(|e| e)
                  .and_then(|plan| Ok(sender.send(plan)))
                  .map_err(|e| e) {
            Ok(_) => info!("Added task"),
            Err(e) => error!("Cannot add task: {}", e),
        };
    }

    Ok(())
}

fn main() {
    let journal_name = get_journal_name();
    let _ = simple_logging::log_to_file(journal_name, LevelFilter::Info);

    info!("Daemon started");

    match bind()
         .map_err(|e| e)
         .and_then(listen)
         .map_err(|e| e) {
        Ok(_) => {},
        Err(e) => error!("Cannot start daemon: {}", e),
    }

    info!("Daemon stopped");
}
