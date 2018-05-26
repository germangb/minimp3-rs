use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InsufficientData,
    SkippedData,
    Eof,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Io(io_err) => write!(f, "IO error: {}", io_err),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Io(io_err) => io_err.description(),
            Error::InsufficientData => "Insufficient data",
            Error::SkippedData => "Skipped data",
            Error::Eof => "End of reader",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            Error::Io(io_err) => Some(io_err),
            _ => None,
        }
    }
}
