# use-listing

Business listing and citation primitives for `RustUse` presence utilities.

## Example

```rust
use use_listing::{ListingName, ListingProfile, ListingProvider, ListingStatus, NapRecord};

let profile = ListingProfile::new(
    ListingName::new("Example Cafe").unwrap(),
    ListingProvider::new("Directory").unwrap(),
)
.with_status(ListingStatus::Claimed)
.with_nap_record(NapRecord::new("Example Cafe", "1 Main St", "+1-555-0100").unwrap());

assert_eq!(profile.status(), ListingStatus::Claimed);
```

## Scope

- Listing names, URLs, providers, citations, name/address/phone records, consistency scoring, statuses, and profiles.

## Non-goals

- API-specific directory integrations, review management, or data synchronization.

## License

Licensed under either Apache-2.0 or MIT, at your option.
