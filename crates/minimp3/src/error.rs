use thiserror::Error;

/// Errors encountered by the MP3 decoder.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    /// An error caused by some IO operation required during decoding.
    Io(#[from] std::io::Error),
    #[error("Insufficient data")]
    /// The decoder tried to parse a frame from its internal buffer, but there
    /// was not enough.
    InsufficientData,
    #[error("Skipped data")]
    /// The decoder encountered data which was not a frame (ie, ID3 data), and
    /// skipped it.
    SkippedData,
    #[error("End of reader")]
    /// The decoder has reached the end of the provided reader.
    Eof,
}
