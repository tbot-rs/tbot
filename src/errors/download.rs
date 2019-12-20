use hyper::StatusCode;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents possible errors whic may occur while downloading a file.
#[derive(Debug)]
#[must_use]
pub enum Download {
    /// The provided file had the `path` field set to `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a different from 200 status code.
    InvalidStatusCode(StatusCode),
}

impl Download {
    /// Checks if `self` is `NoPath`.
    #[must_use]
    pub fn is_no_path(&self) -> bool {
        match self {
            Self::NoPath => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Network`.
    #[must_use]
    pub fn is_network(&self) -> bool {
        match self {
            Self::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InvalidStatusCode`.
    #[must_use]
    pub fn is_invalid_status_code(&self) -> bool {
        match self {
            Self::InvalidStatusCode(..) => true,
            _ => false,
        }
    }
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
    fn from(error: hyper::Error) -> Self {
        Self::Network(error)
    }
}

impl From<StatusCode> for Download {
    fn from(error: StatusCode) -> Self {
        Self::InvalidStatusCode(error)
    }
}
