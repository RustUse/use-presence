# use-metadata

Page metadata and social preview primitives for `RustUse` presence utilities.

## Example

```rust
use use_metadata::{MetadataDescription, MetadataTitle, OpenGraphType, PageMetadata, SocialPreview};

let metadata = PageMetadata::new(
    MetadataTitle::new("Example Article").unwrap(),
    MetadataDescription::new("A short example article summary.").unwrap(),
)
.with_social_preview(SocialPreview::new(
    MetadataTitle::new("Example Article").unwrap(),
    MetadataDescription::new("A short example article summary.").unwrap(),
).with_open_graph_type(OpenGraphType::Article));

assert!(metadata.social_preview().is_some());
```

## Scope

- Metadata titles, descriptions, Open Graph types and images, Twitter card kinds, social previews, and page metadata records.

## Non-goals

- Platform SDKs, HTML rendering, crawler behavior, or image processing.

## License

Licensed under either Apache-2.0 or MIT, at your option.
