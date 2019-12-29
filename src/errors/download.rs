use hyper::StatusCode;
use is_macro::Is;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents possible errors which may occur while downloading a file.
#[derive(Debug, Is)]
pub enum Download {
    /// The provided file had the `path` field set to `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a different from 200 status code.
    InvalidStatusCode(StatusCode),
}

impl Display for Download {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoPath => write!(
                formatter,
                "A file could not be downloaded because of missing `path`.",
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
