# use-presence

Facade crate for `RustUse` external digital presence primitives.

`use-presence` re-exports the focused crates in the presence set. Implementation lives in child crates such as `use-seo`, `use-local`, `use-listing`, `use-metadata`, `use-utm`, `use-referrer`, and `use-attribution`.

## Example

```rust
use use_presence::prelude::*;

let params = UtmParameters::new(
    UtmSource::new("newsletter").unwrap(),
    UtmMedium::new("email").unwrap(),
    UtmCampaign::new("spring-launch").unwrap(),
);

assert_eq!(params.campaign().as_str(), "spring-launch");
```

## Scope

- Crate-shaped module aliases for every focused presence crate.
- A curated prelude for common primitive types.
- Composition examples that show how focused crates fit together.

## Non-goals

- Implementation logic in the facade.
- Crawling, network calls, platform SDKs, analytics systems, or marketing automation.

## License

Licensed under either Apache-2.0 or MIT, at your option.