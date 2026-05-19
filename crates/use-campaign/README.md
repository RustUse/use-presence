# use-campaign

Campaign identity primitives for `RustUse` presence utilities.

## Example

```rust
use use_campaign::{CampaignChannel, CampaignFlight, CampaignId, CampaignName, CampaignStatus};

let id = CampaignId::new("spring-2026").unwrap();
let name = CampaignName::new("Spring Launch").unwrap();
let flight = CampaignFlight::new("2026-03-01").unwrap().with_end_label("2026-03-31").unwrap();

assert_eq!(CampaignStatus::Active.as_str(), "active");
assert_eq!(CampaignChannel::Email.as_str(), "email");
assert_eq!(flight.start_label(), "2026-03-01");
assert_eq!(id.as_str(), "spring-2026");
assert_eq!(name.as_str(), "Spring Launch");
```

## Scope

- Campaign IDs, names, channels, media labels, flights, statuses, and generic labels.

## Non-goals

- Ads platform integrations, bidding, budgets, analytics, or automation.

## License

Licensed under either Apache-2.0 or MIT, at your option.
