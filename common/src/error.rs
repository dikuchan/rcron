use std::fmt;

pub type JobResult<T> = Result<T, JobError>;

#[derive(Debug)]
pub enum JobError {
    TransferAborted(String),
    AccessDenied((u32, u32)),
    TimePassed(i64),
}

impl From<bincode::Error> for JobError {
    fn from(_: bincode::Error) -> Self {
        let message = "cannot transfer data through a socket";

        Self::TransferAborted(message.to_owned())
    }
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
