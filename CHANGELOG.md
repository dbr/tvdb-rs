# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

Change log format based on
["Keep a CHANGELOG"](http://keepachangelog.com/).

## [Unreleased]
- Better error handling - library should no longer panic in any reasonable case.

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
