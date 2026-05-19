# use-geo

Geographic presence and discovery targeting primitives for `RustUse`.

## Example

```rust
use use_geo::{GeoPoint, GeoRegionCode, GeoTarget, LocationRadius, ServiceArea};

let center = GeoPoint::new(40.7128, -74.0060).unwrap();
let radius = LocationRadius::from_kilometers(25.0).unwrap();
let region = GeoRegionCode::new("us-ny").unwrap();
let area = ServiceArea::new("New York metro").unwrap().with_region(region);
let target = GeoTarget::Radius { center, radius };

assert!(matches!(target, GeoTarget::Radius { .. }));
assert_eq!(area.label(), "New York metro");
```

## Scope

- `GeoPoint`, region codes, service-area labels, location radius values, and target descriptors.
- Presence and discovery labels for external surfaces.

## Non-goals

- GIS, projections, geocoding, routing, or geometry math.
- Replacing `use-geography` or `use-geometry`.

## License

Licensed under either Apache-2.0 or MIT, at your option.