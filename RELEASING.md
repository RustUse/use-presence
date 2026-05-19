# Releasing

This repository uses a focused-first release flow.

## Current release state

`use-presence` publishes focused primitive crates first, then the `use-presence` facade crate after the focused crates resolve from crates.io.

## Canonical release guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for first-wave publish scope, focused-crate publish ordering, and publish readiness checks.

## Local validation

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
