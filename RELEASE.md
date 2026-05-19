# Release Policy

The root workspace metadata keeps `publish = false` as the default, while the publishable crate manifests under `crates/` opt in with `publish = true`.

## First Publish Wave

Publish all focused crates first, then publish the `use-presence` facade after crates.io index propagation:

1. `use-seo`
2. `use-geo`
3. `use-local`
4. `use-listing`
5. `use-sitemap`
6. `use-robots`
7. `use-canonical`
8. `use-metadata`
9. `use-schema`
10. `use-utm`
11. `use-campaign`
12. `use-referrer`
13. `use-attribution`
14. `use-presence`

## Publish Surface

- Keep the workspace-level default at `publish = false`.
- Keep focused crates and the facade crate under `crates/` at `publish = true` only after review.
- Keep the facade crate thin and implementation-free.

## Versioning

- The workspace uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
7. Confirm `cargo deny check` and `cargo audit` pass when optional tools are installed.
8. Review README examples, crate metadata, `Cargo.lock`, and changelog entries.
9. Publish focused crates first, wait for crates.io index propagation, then dry-run and publish `use-presence`.
