use use_presence::prelude::{UtmCampaign, UtmMedium, UtmParameters, UtmSource, UtmUrl};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = UtmParameters::new(
        UtmSource::new("newsletter")?,
        UtmMedium::new("email")?,
        UtmCampaign::new("spring-launch")?,
    );
    let url = UtmUrl::new("https://example.com/pricing", parameters)?;

    assert_eq!(
        url.to_string(),
        "https://example.com/pricing?utm_source=newsletter&utm_medium=email&utm_campaign=spring-launch"
    );
    Ok(())
}
