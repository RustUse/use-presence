# use-schema

Practical structured-data primitives for `RustUse` presence utilities.

## Example

```rust
use use_schema::{Organization, PostalAddress};

let address = PostalAddress::new("1 Main St", "Portland", "OR", "97201", "US").unwrap();
let organization = Organization::new("Example Co").unwrap().with_address(address);

assert_eq!(organization.schema_type(), "Organization");
```

## Scope

- Practical records for organizations, local businesses, products, articles, breadcrumbs, FAQ entries, addresses, ratings, and opening hours.

## Non-goals

- A full schema.org implementation, JSON-LD serialization framework, or validation service.

## License

Licensed under either Apache-2.0 or MIT, at your option.
