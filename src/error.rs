use std::error::Error as StdError;
use std::fmt;
use std::io;

/// Errors encountered by the MP3 decoder.
#[derive(Debug)]
pub enum Error {
    /// An error caused by some IO operation required during decoding.
    Io(io::Error),
    /// The decoder tried to parse a frame from its internal buffer, but there was not enough.
    InsufficientData,
    /// The decoder encountered data which was not a frame (ie, ID3 data), and skipped it.
    SkippedData,
    /// The decoder has reached the end of the provided reader.
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
