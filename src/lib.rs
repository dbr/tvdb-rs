//! Module to access [the API][apidoc] of [TheTVDB.com][tvdb]
//!
//! [apidoc]: http://www.thetvdb.com/wiki/index.php/Programmers_API
//! [tvdb]: http://thetvdb.com

extern crate log;

extern crate reqwest;
extern crate url;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Module structure
mod error;
mod data;
pub mod api;

// Main public API
pub use api::{RequestClient, Tvdb};

// Expose error types
pub use error::{TvdbError, TvdbResult};

// Expose data types
pub use data::{Date, EpisodeId};
