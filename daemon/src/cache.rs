use crate::Scheduler;

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
};

use chrono::Local;

/// Allows caching.
pub trait Cache {
    fn load<P: AsRef<Path>>(path: &P) -> Result<Self, RecordError> 
        where Self: Sized;

    fn save<P: AsRef<Path>>(&self, path: &P) -> Result<(), RecordError> 
        where Self: Sized;
}

impl Cache for Scheduler {
    fn load<P: AsRef<Path>>(path: &P) -> Result<Self, RecordError> {
        let file = File::open(path)?;
        let mut buffer = BufReader::new(file);
        let mut scheduler: Self = bincode::deserialize_from(&mut buffer)?;
        let now = Local::now().timestamp();

        // Remove unnecessary tasks.
        scheduler.retain(|&time, _| time >= now as u64);

        Ok(scheduler)
    }

    fn save<P: AsRef<Path>>(&self, path: &P) -> Result<(), RecordError> {
        let file = match OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path) {
            Ok(f) => f,
            Err(_) => File::open(path)?,
        };
        let mut buffer = BufWriter::new(file);

        Ok(bincode::serialize_into(&mut buffer, &self)?)
    }
}

#[derive(Debug)]
pub struct RecordError(String);

impl From<bincode::Error> for RecordError {
    fn from(e: bincode::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<std::io::Error> for RecordError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}