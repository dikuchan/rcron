use std::fmt;

pub type DaemonResult<T> = Result<T, DaemonError>;

#[derive(Debug)]
pub struct DaemonError(String);

impl From<std::io::Error> for DaemonError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
