# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

Change log format based on
["Keep a CHANGELOG"](http://keepachangelog.com/).

## [0.6.0] - 2020-08-08
- Updated `request` to version 0.10 - ([PR #3](https://github.com/dbr/tvdb-rs/pull/3))
- Invalid language error should be a string, not vec-of-strings - ([PR #2](https://github.com/dbr/tvdb-rs/pull/4))

## [0.5.1] - 2018-12-06
- Updated internal requirements including `reqwest` ([Issue #2](https://github.com/dbr/tvdb-rs/issues/2))

## [0.5.0] - 2018-09-28
- Significant rework to use new TVDB API version 2. Still very much a work-in-progress

## [0.4.0] - 2016-07-17
- Better error handling - library should no longer panic in any reasonable case.
- Additional code in `examples`

## [0.3.1] - 2016-07-08
- Explicit dependency on `log` version to keep [Cargo happy](http://doc.crates.io/faq.html#can-libraries-use--as-a-version-for-their-dependencies)

## [0.3.0] - 2016-07-08
- No longer panics on invalid XML, returns descriptive
  TvdbError::CommunicationError for HTTP errors and various other improved
  error handling improvements.
- Upgrade various dependencies (mainly hyper to 0.9, xmltree to 0.3, url to 1.1)

## [0.2.0] - 2015-12-25
- [Added] Exposed all available fields on EpisodeInfo
- [Changed] Uses log module for debug messages

## [0.1.0] - 2015-12-21
- Initial release
