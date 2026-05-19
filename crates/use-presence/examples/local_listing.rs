use use_presence::prelude::{
    BusinessPresenceKind, ListingName, ListingProfile, ListingProvider, ListingStatus, NapRecord,
    ServiceAreaBusiness,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let business = ServiceAreaBusiness::new("Example Cafe", BusinessPresenceKind::Storefront)?;
    let profile = ListingProfile::new(
        ListingName::new(business.name())?,
        ListingProvider::new("Directory")?,
    )
    .with_status(ListingStatus::Claimed)
    .with_nap_record(NapRecord::new("Example Cafe", "1 Main St", "+1-555-0100")?);

    assert_eq!(profile.status(), ListingStatus::Claimed);
    Ok(())
}
