use std::fmt;

pub type EPResult<T> = Result<T, EPError>;

#[derive(Debug)]
pub enum EPError {
    TransferAborted(String),
    AccessDenied((u32, u32)),
    TimePassed(i64),
}

impl From<bincode::Error> for EPError {
    fn from(_: bincode::Error) -> Self {
        let message = "cannot transfer data through a socket";

        Self::TransferAborted(message.to_string())
    }
}

impl fmt::Display for EPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
