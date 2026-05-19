# use-seo

SEO and search snippet primitives for `RustUse` presence utilities.

## Example

```rust
use use_seo::{IndexingHint, MetaDescription, PageIntent, SearchSnippetMetadata, SeoTitle};

let snippet = SearchSnippetMetadata::new(
    SeoTitle::new("Example Services").unwrap(),
    MetaDescription::new("Helpful service details for Example.").unwrap(),
)
.with_intent(PageIntent::Local)
.with_indexing_hint(IndexingHint::Index);

assert_eq!(snippet.intent(), PageIntent::Local);
```

## Scope

- Titles, meta descriptions, slug hints, indexing hints, link relation hints, page intent, and snippet metadata.
- Lightweight validation and labels for composing external presence metadata.

## Non-goals

- Crawling, ranking, search engine clients, or SEO automation.
- Full HTML document generation.

## License

Licensed under either Apache-2.0 or MIT, at your option.