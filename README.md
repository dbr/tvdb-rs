# `tvdb-rs`

[![Build Status](https://travis-ci.org/dbr/tvdb-rs.png?branch=master)](https://travis-ci.org/dbr/tvdb-rs)
[![Crates.io link](https://img.shields.io/crates/v/tvdb.svg)](https://crates.io/crates/tvdb)
[![Docs.rs link](https://docs.rs/tvdb/badge.svg)](https://docs.rs/tvdb)

[TheTVDB.com][tvdb] interface for the Rust programming langauge

[tvdb]: http://thetvdb.com/


## Status

0.5.0 will be first version to support v2 TVDB API. 0.4.0 is non-functional

## Release procedure
1. Make changes
2. Ensure CHANGELOG.md is updated
3. `cargo test` etc
4. Bump version in Cargo.toml
5. Commit version bump
6. `cargo publish` pushes new version to cargo
7. Tag release `git tag -a v0.1.0`
8. `git push --tags`
