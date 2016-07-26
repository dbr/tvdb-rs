//! Module to access [the API][apidoc] of [TheTVDB.com][tvdb]
//!
//! [apidoc]: http://www.thetvdb.com/wiki/index.php/Programmers_API
//! [tvdb]: http://thetvdb.com

#[macro_use]
extern crate log;

extern crate xmltree;
extern crate hyper;
extern crate url;

// Module structure
mod parse;
mod error;
mod data;
mod api;

// Main public API
pub use api::Tvdb;

// Expose error types
pub use error::{TvdbError, TvdbResult};

// Expose data types
pub use data::{
    Date,
    EpisodeId,
    SeriesSearchResult,
    EpisodeInfo,
};
