# Release procedure

1. Make changes
2. Ensure CHANGELOG.md is updated
3. `cargo test` etc
4. Bump version in Cargo.toml
5. Commit version bump
6. `cargo publish` pushes new version to cargo
7. Tag release `git tag -a v0.1.0`
8. `git push --tags`
