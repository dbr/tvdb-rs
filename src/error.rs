use std::fmt;
use std::num::ParseIntError;

/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    InternalError{reason: String},
    SeriesNotFound,
    CommunicationError{reason: String},
    DataError{reason: String},
    Cancelled,
}

/// Shortcut for some type wrapped in a `TvdbError`
pub type TvdbResult<T> = Result<T, TvdbError>;

/// Formatting for error
impl fmt::Display for TvdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvdbError::InternalError{reason: ref e} => write!(f, "Internal error: {}", e),
            TvdbError::SeriesNotFound => write!(f, "Series not found"),
            TvdbError::CommunicationError{reason: ref e} => write!(f, "Communication error: {}", e),
            TvdbError::DataError{reason: ref e} => write!(f, "Data error: {}", e),
            TvdbError::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Convert from parse error (e.g for dateify() function)
impl From<ParseIntError> for TvdbError{
    fn from(err: ParseIntError) -> TvdbError{
        TvdbError::DataError{reason: format!("{}", err)} // FIXME
    }
}
