use use_presence::prelude::{
    CanonicalUrl, MetadataDescription, MetadataTitle, PageMetadata, RobotsDirective,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let canonical = CanonicalUrl::new("https://example.com/services")?;
    let robots = RobotsDirective::index_follow();
    let metadata = PageMetadata::new(
        MetadataTitle::new("Example Services")?,
        MetadataDescription::new("Helpful service details for Example.")?,
    )
    .with_canonical_url(canonical.as_str())
    .with_robots(robots.to_meta_content());

    assert_eq!(metadata.robots(), Some("index,follow"));
    Ok(())
}
