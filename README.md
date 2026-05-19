# RustUse/use-presence

Composable primitives for search, local discovery, listings, metadata, campaigns, attribution, and external digital presence.

`use-presence` is the RustUse set for how an entity, page, location, campaign, or business is discovered, represented, indexed, attributed, and reached across external digital surfaces. It is a primitive Rust utility workspace, not an application, crawler, analytics system, ads client, or marketing automation platform.

## Current status

- Initial `0.0.1` workspace scaffold.
- Rust edition 2024 and Rust 1.95.0.
- Dual licensed as `MIT OR Apache-2.0`.
- Focused crates are dependency-light and can be used directly.
- The top-level `use-presence` crate is a facade/re-export crate only.

## Workspace crates

| Crate             | Purpose                                                                                     |
| ----------------- | ------------------------------------------------------------------------------------------- |
| `use-presence`    | Facade crate that re-exports the focused presence crates.                                   |
| `use-seo`         | SEO labels for titles, descriptions, index hints, page intent, link relation, and snippets. |
| `use-geo`         | Geographic presence labels for discovery and targeting.                                     |
| `use-local`       | Local business presence primitives and visibility hints.                                    |
| `use-listing`     | Business listing, citation, and name/address/phone consistency primitives.                  |
| `use-sitemap`     | Sitemap URL, entry, priority, change frequency, and XML-safe formatting helpers.            |
| `use-robots`      | Robots and indexing directive primitives.                                                   |
| `use-canonical`   | Canonical URL, alternate URL, redirect, hreflang, and duplicate-surface hints.              |
| `use-metadata`    | Page metadata, Open Graph, Twitter card, and social preview primitives.                     |
| `use-schema`      | Practical schema-like primitive records for common external surfaces.                       |
| `use-utm`         | UTM parameter parsing, validation, and formatting helpers.                                  |
| `use-campaign`    | Campaign identity, channel, medium, flight, status, and label primitives.                   |
| `use-referrer`    | Referrer/source label parsing and lightweight traffic classification.                       |
| `use-attribution` | Attribution source, medium, touchpoint, window, credit, model-kind, and conversion labels.  |

## Installation

Use the facade crate when one dependency and one import surface are clearer:

```toml
[dependencies]
use-presence = "0.0.1"
```

Use focused crates when you want the narrowest dependency surface:

```toml
[dependencies]
use-utm = "0.0.1"
use-robots = "0.0.1"
use-listing = "0.0.1"
```

## Examples

### Metadata with canonical and robots hints

```rust
use use_canonical::CanonicalUrl;
use use_metadata::{MetadataDescription, MetadataTitle, PageMetadata};
use use_robots::RobotsDirective;

let canonical = CanonicalUrl::new("https://example.com/services").unwrap();
let robots = RobotsDirective::index_follow();
let metadata = PageMetadata::new(
    MetadataTitle::new("Example Services").unwrap(),
    MetadataDescription::new("Service information for Example.").unwrap(),
)
.with_canonical_url(canonical.as_str())
.with_robots(robots.to_meta_content());

assert_eq!(metadata.canonical_url(), Some("https://example.com/services"));
```

### UTM campaign URL

```rust
use use_utm::{UtmCampaign, UtmMedium, UtmParameters, UtmSource, UtmUrl};

let params = UtmParameters::new(
    UtmSource::new("newsletter").unwrap(),
    UtmMedium::new("email").unwrap(),
    UtmCampaign::new("spring-launch").unwrap(),
);
let url = UtmUrl::new("https://example.com/pricing", params).unwrap();

assert_eq!(url.to_string(), "https://example.com/pricing?utm_source=newsletter&utm_medium=email&utm_campaign=spring-launch");
```

## Scope

- Typed labels and small value objects for external presence surfaces.
- Simple validation, normalization, parsing, and formatting.
- Builders and composition helpers that stay useful without platform integrations.
- Unit-testable primitives that downstream applications can combine.

## Non-goals

- Web crawling, scraping, or search ranking.
- HTTP fetching, database storage, or UI components.
- Google Business Profile, Google Analytics, ads platform, or social platform clients.
- Full schema.org, WHATWG URL, XML, analytics, or attribution-model implementations.
- Business-specific expansive logic or marketing automation workflows.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
