use hyper::StatusCode;
use is_macro::Is;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};

/// Represents possible errors which may occur while downloading a file.
#[derive(Debug, Is)]
pub enum Download {
    /// The provided file's `path` is `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a status code different from `200`.
    InvalidStatusCode(StatusCode),
    /// Failed to read a local file. This can only be returned if you use
    /// a self-hosted Bot API server.
    Io(io::Error),
}

impl Display for Download {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoPath => write!(
                formatter,
                "A file could not be downloaded because of a missing `path`.",
            ),
            Self::Network(error) => write!(
                formatter,
                "A file could not be downloaded because of a network error: {}",
                error,
            ),
            Self::InvalidStatusCode(code) => write!(
                formatter,
                "A file could not be downloaded because Telegram responded \
                 with {} instead of 200 OK.",
                code,
            ),
            Self::Io(error) => write!(
                formatter,
                "The server returned a local path to the file, but an error \
                 occured reading it: {}",
                error
            ),
        }
    }
}

impl Error for Download {}

impl From<hyper::Error> for Download {
    #[must_use]
    fn from(error: hyper::Error) -> Self {
        Self::Network(error)
    }
}

impl From<StatusCode> for Download {
    #[must_use]
    fn from(error: StatusCode) -> Self {
        Self::InvalidStatusCode(error)
    }
}

impl From<io::Error> for Download {
    #[must_use]
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
