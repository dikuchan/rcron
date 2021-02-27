use crate::{
    error::{JobError, JobResult},
    geteuid, getegid
};

use std::{
    os::unix::net::UnixStream,
};

use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub uid: u32,
    pub gid: u32,
    pub command: String,
    pub args: Vec<String>,
    pub time: u64,
}

impl Job {
    pub fn new(command: &str, args: Vec<&str>, time: i64) -> JobResult<Self> {
        // `man getuid`: These functions are always successful.
        let uid = unsafe { geteuid() };
        let gid = unsafe { getegid() };
        if uid < 1000 || gid < 1000 {
            return Err(JobError::AccessDenied((uid, gid)));
        }

        let now = Local::now().timestamp();
        if time - now < 1 {
            return Err(JobError::TimePassed(time));
        }

        Ok(Self {
            uid,
            gid,
            command: command.to_string(),
            args: args.into_iter().map(String::from).collect(),
            time: time as u64,
        })
    }

    pub fn send(&self, stream: UnixStream) -> JobResult<()> {
        Ok(bincode::serialize_into(stream, &self)?)
    }

    pub fn receive(stream: UnixStream) -> JobResult<Self> {
        Ok(bincode::deserialize_from(stream)?)
    }
}
