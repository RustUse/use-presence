# use-canonical

Canonical URL and duplicate-surface primitives for `RustUse`.

## Example

```rust
use use_canonical::{AlternateUrl, CanonicalGroup, CanonicalUrl, DuplicateSurfaceHint, HreflangTag};

let group = CanonicalGroup::new(CanonicalUrl::new("https://example.com/en/").unwrap())
    .with_alternate(AlternateUrl::new("https://example.com/es/").unwrap().with_hreflang(HreflangTag::new("es").unwrap()))
    .with_hint(DuplicateSurfaceHint::LocaleVariant);

assert_eq!(group.alternates().len(), 1);
```

## Scope

- Canonical URLs, alternate URLs, hreflang tags, redirect labels, canonical groups, and duplicate-surface hints.

## Non-goals

- Crawling, redirect execution, HTTP fetching, or full URL standards compliance.

## License

Licensed under either Apache-2.0 or MIT, at your option.
