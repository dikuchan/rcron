use std::fmt;

use common::error::EPError;

pub type ClientResult<T> = Result<T, ClientError>; 

#[derive(Debug)]
pub enum ClientError {
    AccessDenied((u32, u32)),
    InvalidTimeParse(String),
    TimePassed(i64),
    TransferAborted(String),
}

impl From<chrono::ParseError> for ClientError {
    fn from(_: chrono::ParseError) -> Self {
        let message = "cannot parse the provided datetime";

        Self::InvalidTimeParse(message.to_string())
    }
}

impl From<EPError> for ClientError {
    fn from(e: EPError) -> Self {
        match e {
            EPError::AccessDenied(v) => Self::AccessDenied(v),
            EPError::TimePassed(v) => Self::TimePassed(v),
            EPError::TransferAborted(v) => Self::TransferAborted(v),
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(_: std::io::Error) -> Self {
        let message = "cannot transfer data through a socket";

        Self::TransferAborted(message.to_string())
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::AccessDenied((uid, gid)) => 
                write!(f, "only non-system users are allowed ({} > 999, {} > 999)", uid, gid),
            Self::InvalidTimeParse(message) => write!(f, "{}", message),
            Self::TimePassed(time) => 
                write!(f, "time has already passed ({}s)", time.abs()),
            Self::TransferAborted(message) => write!(f, "{}", message),
        }
    }
}
