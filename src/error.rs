use std::fmt;
use std::num::ParseIntError;
use serde_json;
use std::error::Error;

/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    /// An internal error within this library
    InternalError { reason: String },

    /// When looking up a nonexistent series
    SeriesNotFound,

    /// Error contacting TheTVDB.com (e.g HTTP error)
    CommunicationError { reason: String },

    /// Malformed data in response from TheTVDB.com
    DataError { reason: String },

    /// User cancelled in some interactive fashion
    Cancelled,
}

/// Shortcut for some type wrapped in a `TvdbError`
pub type TvdbResult<T> = Result<T, TvdbError>;

/// Formatting for error
impl fmt::Display for TvdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvdbError::InternalError { reason: ref e } => write!(f, "Internal error: {}", e),
            TvdbError::SeriesNotFound => write!(f, "Series not found"),
            TvdbError::CommunicationError { reason: ref e } => {
                write!(f, "Communication error: {}", e)
            }
            TvdbError::DataError { reason: ref e } => write!(f, "Data error: {}", e),
            TvdbError::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Convert from parse error (e.g for dateify() function)
impl From<ParseIntError> for TvdbError {
    fn from(err: ParseIntError) -> TvdbError {
        TvdbError::DataError {
            reason: format!("{}", err),
        } // FIXME
    }
}

impl From<serde_json::Error> for TvdbError {
    fn from(err: serde_json::Error) -> TvdbError {
        TvdbError::DataError {
            reason: format!("Error parsing JSON data: {}", err),
        }
    }
}

impl Error for TvdbError {
    fn description(&self) -> &str {
        "TvdbError"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
