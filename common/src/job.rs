use crate::{
    error::{JobError, JobResult},
    get_gid, get_uid,
};

use std::os::unix::net::UnixStream;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub command: String,
    pub args: Vec<String>,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobWithCredentials {
    pub uid: u32,
    pub gid: u32,
    pub command: String,
    pub args: Vec<String>,
    pub time: u64,
}

impl Job {
    pub fn new(command: &str, args: Vec<&str>, time: i64) -> JobResult<Self> {
        let uid = get_uid();
        let gid = get_gid();
        if uid < 1000 || gid < 1000 {
            return Err(JobError::AccessDenied((uid, gid)));
        }

        let now = Local::now().timestamp();
        if time - now < 1 {
            return Err(JobError::TimePassed(time));
        }

        Ok(Self {
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
