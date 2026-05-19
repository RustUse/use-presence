# use-sitemap

Sitemap entry primitives and XML-safe formatting helpers for `RustUse`.

## Example

```rust
use use_sitemap::{ChangeFrequency, Priority, SitemapEntry, SitemapUrl};

let entry = SitemapEntry::new(SitemapUrl::new("https://example.com/").unwrap())
    .with_change_frequency(ChangeFrequency::Weekly)
    .with_priority(Priority::new(0.8).unwrap());

assert!(entry.to_url_xml().contains("<changefreq>weekly</changefreq>"));
```

## Scope

- Sitemap URLs, entries, index entries, change frequency, priority, last-modified labels, and XML escaping.

## Non-goals

- A full XML framework, file IO, crawling, or URL discovery.

## License

Licensed under either Apache-2.0 or MIT, at your option.
