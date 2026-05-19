# use-attribution

Attribution and touchpoint label primitives for `RustUse` presence utilities.

## Example

```rust
use use_attribution::{AttributionCredit, AttributionMedium, AttributionSource, Touchpoint};

let touchpoint = Touchpoint::new(
    AttributionSource::new("newsletter").unwrap(),
    AttributionMedium::new("email").unwrap(),
)
.with_credit(AttributionCredit::new(0.5).unwrap());

assert_eq!(touchpoint.credit().unwrap().value(), 0.5);
```

## Scope

- Touchpoints, attribution source and medium labels, windows, credit values, model-kind labels, and conversion labels.

## Non-goals

- Full analytics models, conversion tracking, probabilistic modeling, or data pipelines.

## License

Licensed under either Apache-2.0 or MIT, at your option.