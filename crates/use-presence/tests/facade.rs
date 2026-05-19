use use_presence::prelude::{
    BusinessPresenceKind, CanonicalUrl, ListingName, ListingProfile, ListingProvider,
    MetadataDescription, MetadataTitle, PageMetadata, ReferrerKind, RobotsDirective,
    ServiceAreaBusiness, UtmCampaign, UtmMedium, UtmParameters, UtmSource, classify_source_medium,
};

#[test]
fn facade_exposes_page_metadata_composition() {
    let canonical = CanonicalUrl::new("https://example.com/services").unwrap();
    let robots = RobotsDirective::index_follow();
    let metadata = PageMetadata::new(
        MetadataTitle::new("Example Services").unwrap(),
        MetadataDescription::new("Helpful service metadata.").unwrap(),
    )
    .with_canonical_url(canonical.as_str())
    .with_robots(robots.to_meta_content());

    assert_eq!(
        metadata.canonical_url(),
        Some("https://example.com/services")
    );
    assert_eq!(metadata.robots(), Some("index,follow"));
}

#[test]
fn facade_exposes_local_listing_composition() {
    let business =
        ServiceAreaBusiness::new("Example Plumbing", BusinessPresenceKind::ServiceArea).unwrap();
    let listing = ListingProfile::new(
        ListingName::new(business.name()).unwrap(),
        ListingProvider::new("Directory").unwrap(),
    );

    assert!(business.is_service_area_business());
    assert_eq!(listing.name().as_str(), "Example Plumbing");
}

#[test]
fn facade_exposes_campaign_and_referrer_composition() {
    let parameters = UtmParameters::new(
        UtmSource::new("newsletter").unwrap(),
        UtmMedium::new("email").unwrap(),
        UtmCampaign::new("spring").unwrap(),
    );

    assert_eq!(
        parameters.to_query_string(),
        "utm_source=newsletter&utm_medium=email&utm_campaign=spring"
    );
    assert_eq!(
        classify_source_medium(Some("newsletter"), Some("email")),
        ReferrerKind::Email
    );
}
