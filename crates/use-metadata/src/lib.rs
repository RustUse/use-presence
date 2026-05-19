#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty(value: impl AsRef<str>, field: &'static str) -> Result<String, MetadataValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(MetadataValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

fn is_http_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://")
}

/// Error returned by metadata primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetadataValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// The URL did not look like an HTTP or HTTPS URL.
    InvalidUrl,
    /// Image dimensions must be non-zero.
    InvalidImageDimensions,
}

impl fmt::Display for MetadataValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidUrl => {
                formatter.write_str("metadata URL must start with http:// or https://")
            },
            Self::InvalidImageDimensions => {
                formatter.write_str("image dimensions must be non-zero")
            },
        }
    }
}

impl Error for MetadataValueError {}

/// Page metadata title.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataTitle(String);

impl MetadataTitle {
    /// Creates a metadata title.
    ///
    /// # Errors
    ///
    /// Returns [`MetadataValueError::Empty`] when the title is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, MetadataValueError> {
        non_empty(value, "metadata title").map(Self)
    }

    /// Returns the title text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MetadataTitle {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MetadataTitle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MetadataTitle {
    type Err = MetadataValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Page metadata description.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataDescription(String);

impl MetadataDescription {
    /// Creates a metadata description.
    ///
    /// # Errors
    ///
    /// Returns [`MetadataValueError::Empty`] when the description is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, MetadataValueError> {
        non_empty(value, "metadata description").map(Self)
    }

    /// Returns the description text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MetadataDescription {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MetadataDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Open Graph type label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OpenGraphType {
    /// Generic website.
    Website,
    /// Article surface.
    Article,
    /// Product surface.
    Product,
    /// Profile surface.
    Profile,
    /// Local business surface.
    LocalBusiness,
}

impl OpenGraphType {
    /// Returns the Open Graph type label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Website => "website",
            Self::Article => "article",
            Self::Product => "product",
            Self::Profile => "profile",
            Self::LocalBusiness => "business.business",
        }
    }
}

/// Twitter card kind label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TwitterCardKind {
    /// Summary card.
    Summary,
    /// Summary card with a large image.
    SummaryLargeImage,
    /// App card.
    App,
    /// Player card.
    Player,
}

impl TwitterCardKind {
    /// Returns the card kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::SummaryLargeImage => "summary_large_image",
            Self::App => "app",
            Self::Player => "player",
        }
    }
}

/// Open Graph image metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenGraphImage {
    url: String,
    alt: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

impl OpenGraphImage {
    /// Creates an Open Graph image from a URL-like string.
    ///
    /// # Errors
    ///
    /// Returns [`MetadataValueError::InvalidUrl`] when the URL shape is unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, MetadataValueError> {
        let url = non_empty(value, "Open Graph image URL")?;
        if !is_http_url(&url) {
            return Err(MetadataValueError::InvalidUrl);
        }

        Ok(Self {
            url,
            alt: None,
            width: None,
            height: None,
        })
    }

    /// Sets alternative text.
    ///
    /// # Errors
    ///
    /// Returns [`MetadataValueError::Empty`] when the alt text is empty.
    pub fn with_alt(mut self, alt: impl AsRef<str>) -> Result<Self, MetadataValueError> {
        self.alt = Some(non_empty(alt, "Open Graph image alt text")?);
        Ok(self)
    }

    /// Sets image dimensions.
    ///
    /// # Errors
    ///
    /// Returns [`MetadataValueError::InvalidImageDimensions`] when either dimension is zero.
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Result<Self, MetadataValueError> {
        if width == 0 || height == 0 {
            return Err(MetadataValueError::InvalidImageDimensions);
        }
        self.width = Some(width);
        self.height = Some(height);
        Ok(self)
    }

    /// Returns the image URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// Social preview metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SocialPreview {
    title: MetadataTitle,
    description: MetadataDescription,
    image: Option<OpenGraphImage>,
    open_graph_type: OpenGraphType,
    twitter_card: TwitterCardKind,
}

impl SocialPreview {
    /// Creates a social preview with default website/summary labels.
    #[must_use]
    pub const fn new(title: MetadataTitle, description: MetadataDescription) -> Self {
        Self {
            title,
            description,
            image: None,
            open_graph_type: OpenGraphType::Website,
            twitter_card: TwitterCardKind::Summary,
        }
    }

    /// Sets the preview image.
    #[must_use]
    pub fn with_image(mut self, image: OpenGraphImage) -> Self {
        self.image = Some(image);
        self
    }

    /// Sets the Open Graph type.
    #[must_use]
    pub const fn with_open_graph_type(mut self, value: OpenGraphType) -> Self {
        self.open_graph_type = value;
        self
    }

    /// Sets the Twitter card kind.
    #[must_use]
    pub const fn with_twitter_card(mut self, value: TwitterCardKind) -> Self {
        self.twitter_card = value;
        self
    }

    /// Returns the Open Graph type.
    #[must_use]
    pub const fn open_graph_type(&self) -> OpenGraphType {
        self.open_graph_type
    }
}

/// Page metadata for external surfaces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageMetadata {
    title: MetadataTitle,
    description: MetadataDescription,
    canonical_url: Option<String>,
    robots: Option<String>,
    social_preview: Option<SocialPreview>,
}

impl PageMetadata {
    /// Creates page metadata from a title and description.
    #[must_use]
    pub const fn new(title: MetadataTitle, description: MetadataDescription) -> Self {
        Self {
            title,
            description,
            canonical_url: None,
            robots: None,
            social_preview: None,
        }
    }

    /// Sets a canonical URL label.
    #[must_use]
    pub fn with_canonical_url(mut self, url: impl Into<String>) -> Self {
        self.canonical_url = Some(url.into());
        self
    }

    /// Sets robots meta content.
    #[must_use]
    pub fn with_robots(mut self, content: impl Into<String>) -> Self {
        self.robots = Some(content.into());
        self
    }

    /// Sets social preview metadata.
    #[must_use]
    pub fn with_social_preview(mut self, preview: SocialPreview) -> Self {
        self.social_preview = Some(preview);
        self
    }

    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &MetadataTitle {
        &self.title
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &MetadataDescription {
        &self.description
    }

    /// Returns the canonical URL label.
    #[must_use]
    pub fn canonical_url(&self) -> Option<&str> {
        self.canonical_url.as_deref()
    }

    /// Returns robots meta content.
    #[must_use]
    pub fn robots(&self) -> Option<&str> {
        self.robots.as_deref()
    }

    /// Returns social preview metadata.
    #[must_use]
    pub const fn social_preview(&self) -> Option<&SocialPreview> {
        self.social_preview.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MetadataDescription, MetadataTitle, OpenGraphImage, OpenGraphType, PageMetadata,
        SocialPreview, TwitterCardKind,
    };

    #[test]
    fn validates_metadata_strings() {
        assert!(MetadataTitle::new("Example").is_ok());
        assert!(MetadataDescription::new(" ").is_err());
    }

    #[test]
    fn builds_open_graph_images() {
        let image = OpenGraphImage::new("https://example.com/image.png")
            .unwrap()
            .with_alt("Preview image")
            .unwrap()
            .with_dimensions(1200, 630)
            .unwrap();

        assert_eq!(image.url(), "https://example.com/image.png");
        assert!(OpenGraphImage::new("/image.png").is_err());
    }

    #[test]
    fn composes_page_metadata() {
        let preview = SocialPreview::new(
            MetadataTitle::new("Example").unwrap(),
            MetadataDescription::new("Example description").unwrap(),
        )
        .with_open_graph_type(OpenGraphType::Article)
        .with_twitter_card(TwitterCardKind::SummaryLargeImage);
        let metadata = PageMetadata::new(
            MetadataTitle::new("Example").unwrap(),
            MetadataDescription::new("Example description").unwrap(),
        )
        .with_canonical_url("https://example.com/")
        .with_robots("index,follow")
        .with_social_preview(preview);

        assert_eq!(metadata.robots(), Some("index,follow"));
        assert_eq!(
            metadata.social_preview().unwrap().open_graph_type(),
            OpenGraphType::Article
        );
    }
}
