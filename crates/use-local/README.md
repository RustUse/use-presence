# use-local

Local business presence primitives for `RustUse`.

## Example

```rust
use use_local::{BusinessPresenceKind, LocalCategory, LocalVisibilityHint, ServiceAreaBusiness};

let business = ServiceAreaBusiness::new("Example Plumbing", BusinessPresenceKind::ServiceArea)
    .unwrap()
    .with_category(LocalCategory::new("Plumber").unwrap())
    .with_visibility_hint(LocalVisibilityHint::OnSiteService);

assert!(business.is_service_area_business());
```

## Scope

- Business location labels, opening-hours labels, local categories, service-area businesses, and visibility hints.
- Primitive storefront versus service-area descriptors.

## Non-goals

- Business profile APIs, review APIs, maps SDKs, or scheduling systems.

## License

Licensed under either Apache-2.0 or MIT, at your option.
