# `tvdb-rs`

[![Build Status](https://travis-ci.org/dbr/tvdb-rs.png?branch=master)](https://travis-ci.org/dbr/tvdb-rs)
[![Crates.io link](https://img.shields.io/crates/v/tvdb.svg)](https://crates.io/crates/tvdb)
[![Docs.rs link](https://docs.rs/tvdb/badge.svg)](https://docs.rs/tvdb)

[TheTVDB.com][tvdb] interface for the Rust programming langauge

[tvdb]: http://thetvdb.com/


## Status

- [x] Episode data lookup.
- [ ] Lookup by different air orders (dvd, absolute)
- [ ] Access extended actors data
- [ ] Access banners data
- [ ] Access full series record (`all.zip`)
- [ ] User-specific methods (favorites, rating etc)


## Release procedure
1. Make changes
2. Ensure CHANGELOG.md is updated
3. `cargo test` etc
4. Bump version in Cargo.toml
5. `cargo publish` pushes new version to cargo
6. Commit version bump
7. Tag release `git tag -a v0.1.0`
8. `git push --tags`
