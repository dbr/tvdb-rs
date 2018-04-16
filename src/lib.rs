//! Module to access [the API][apidoc] of [TheTVDB.com][tvdb]
//!
//! [apidoc]: https://api.thetvdb.com/swagger
//! [tvdb]: http://thetvdb.com

extern crate log;

extern crate reqwest;
extern crate url;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Module structure
pub mod raw;
mod data;
mod error;

// Main public API
pub use raw::{RequestClient, Tvdb};

// Expose error types
pub use error::{TvdbError, TvdbResult};

// Expose data types
pub use data::EpisodeId;
