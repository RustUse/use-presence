# use-referrer

Referrer and source classification primitives for `RustUse`.

## Example

```rust
use use_referrer::{ReferrerKind, classify_source_medium};

let kind = classify_source_medium(Some("newsletter"), Some("email"));

assert_eq!(kind, ReferrerKind::Email);
```

## Scope

- Referrer URLs and hosts, source kinds, traffic labels, and lightweight source/medium/referrer classification.

## Non-goals

- Analytics models, event collection, user identification, or traffic storage.

## License

Licensed under either Apache-2.0 or MIT, at your option.
