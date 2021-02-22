use crate::Scheduler;

use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

pub trait Record {
    fn load<P: AsRef<Path>>(path: &P) -> Result<Self, RecordError> 
        where Self: Sized;

    fn save<P: AsRef<Path>>(&self, path: &P) -> Result<(), RecordError> 
        where Self: Sized;
}

impl Record for Scheduler {
    fn load<P: AsRef<Path>>(path: &P) -> Result<Self, RecordError> {
        let file = File::open(path)?;
        let mut buffer = BufReader::new(file);

        Ok(bincode::deserialize_from(&mut buffer)?)
    }

    fn save<P: AsRef<Path>>(&self, path: &P) -> Result<(), RecordError> {
        let mut file = File::open(path)?;
        let mut buffer = BufWriter::new(file);

        Ok(bincode::serialize_into(&mut buffer, self)?)
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
