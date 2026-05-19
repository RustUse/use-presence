# use-utm

UTM parameter parsing and formatting primitives for `RustUse`.

## Example

```rust
use use_utm::{UtmCampaign, UtmMedium, UtmParameters, UtmSource};

let params = UtmParameters::new(
    UtmSource::new("newsletter").unwrap(),
    UtmMedium::new("email").unwrap(),
    UtmCampaign::new("spring-launch").unwrap(),
);

assert_eq!(params.to_query_string(), "utm_source=newsletter&utm_medium=email&utm_campaign=spring-launch");
```

## Scope

- UTM source, medium, campaign, term, content, parameter sets, URL formatting, and parsing helpers.

## Non-goals

- Analytics, attribution modeling, HTTP clients, or platform-specific tracking APIs.

## License

Licensed under either Apache-2.0 or MIT, at your option.
