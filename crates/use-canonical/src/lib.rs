#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn is_http_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://")) && value.contains('.')
}

fn validate_url(
    value: impl AsRef<str>,
    field: &'static str,
) -> Result<String, CanonicalValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        return Err(CanonicalValueError::Empty { field });
    }
    if is_http_url(trimmed) {
        Ok(trimmed.to_string())
    } else {
        Err(CanonicalValueError::InvalidUrl)
    }
}

/// Error returned by canonical primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// The URL did not look like an HTTP or HTTPS URL.
    InvalidUrl,
    /// The hreflang tag was unsupported.
    InvalidHreflang,
}

impl fmt::Display for CanonicalValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidUrl => formatter.write_str("URL must start with http:// or https://"),
            Self::InvalidHreflang => formatter.write_str("hreflang tag shape is unsupported"),
        }
    }
}

impl Error for CanonicalValueError {}

/// A canonical URL label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalUrl(String);

impl CanonicalUrl {
    /// Creates a canonical URL.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalValueError`] when the URL is empty or unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CanonicalValueError> {
        validate_url(value, "canonical URL").map(Self)
    }

    /// Returns the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CanonicalUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CanonicalUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CanonicalUrl {
    type Err = CanonicalValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A hreflang tag for alternate URLs.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HreflangTag(String);

impl HreflangTag {
    /// Creates a normalized hreflang tag.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalValueError::InvalidHreflang`] when the shape is unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CanonicalValueError> {
        let trimmed = value.as_ref().trim();
        if trimmed.eq_ignore_ascii_case("x-default") {
            return Ok(Self("x-default".to_string()));
        }

        let parts = trimmed.split('-').collect::<Vec<_>>();
        let valid_language = parts.first().is_some_and(|part| {
            (2..=3).contains(&part.len()) && part.bytes().all(|b| b.is_ascii_alphabetic())
        });
        let valid_region = parts.get(1).is_none_or(|part| {
            (part.len() == 2 && part.bytes().all(|b| b.is_ascii_alphabetic()))
                || (part.len() == 3 && part.bytes().all(|b| b.is_ascii_digit()))
        });

        if parts.len() <= 2 && valid_language && valid_region {
            let mut normalized = parts[0].to_ascii_lowercase();
            if let Some(region) = parts.get(1) {
                normalized.push('-');
                normalized.push_str(&region.to_ascii_uppercase());
            }
            Ok(Self(normalized))
        } else {
            Err(CanonicalValueError::InvalidHreflang)
        }
    }

    /// Returns the normalized hreflang tag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for HreflangTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// An alternate URL with an optional hreflang tag.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlternateUrl {
    url: String,
    hreflang: Option<HreflangTag>,
}

impl AlternateUrl {
    /// Creates an alternate URL.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalValueError`] when the URL is empty or unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CanonicalValueError> {
        Ok(Self {
            url: validate_url(value, "alternate URL")?,
            hreflang: None,
        })
    }

    /// Sets the hreflang tag.
    #[must_use]
    pub fn with_hreflang(mut self, tag: HreflangTag) -> Self {
        self.hreflang = Some(tag);
        self
    }

    /// Returns the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.url
    }

    /// Returns the optional hreflang tag.
    #[must_use]
    pub const fn hreflang(&self) -> Option<&HreflangTag> {
        self.hreflang.as_ref()
    }
}

/// Redirect relationship kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RedirectKind {
    /// Permanent redirect.
    Permanent,
    /// Temporary redirect.
    Temporary,
    /// See-other redirect.
    SeeOther,
    /// Gone surface.
    Gone,
}

/// Duplicate surface hint for canonical grouping.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DuplicateSurfaceHint {
    /// Query parameters can duplicate content.
    QueryParameters,
    /// Trailing slash variants can duplicate content.
    TrailingSlash,
    /// HTTP and HTTPS variants can duplicate content.
    HttpHttps,
    /// `www` and apex host variants can duplicate content.
    WwwNonWww,
    /// Locale variants exist for the same surface.
    LocaleVariant,
    /// Print page variant.
    PrintPage,
    /// Syndicated copy of a surface.
    SyndicatedCopy,
}

/// A canonical URL with alternates and duplicate-surface hints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalGroup {
    canonical: CanonicalUrl,
    alternates: Vec<AlternateUrl>,
    hints: Vec<DuplicateSurfaceHint>,
}

impl CanonicalGroup {
    /// Creates a canonical group.
    #[must_use]
    pub const fn new(canonical: CanonicalUrl) -> Self {
        Self {
            canonical,
            alternates: Vec::new(),
            hints: Vec::new(),
        }
    }

    /// Adds an alternate URL.
    #[must_use]
    pub fn with_alternate(mut self, alternate: AlternateUrl) -> Self {
        self.alternates.push(alternate);
        self
    }

    /// Adds a duplicate-surface hint.
    #[must_use]
    pub fn with_hint(mut self, hint: DuplicateSurfaceHint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Returns the canonical URL.
    #[must_use]
    pub const fn canonical(&self) -> &CanonicalUrl {
        &self.canonical
    }

    /// Returns alternate URLs.
    #[must_use]
    pub fn alternates(&self) -> &[AlternateUrl] {
        &self.alternates
    }

    /// Returns duplicate-surface hints.
    #[must_use]
    pub fn hints(&self) -> &[DuplicateSurfaceHint] {
        &self.hints
    }
}

#[cfg(test)]
mod tests {
    use super::{AlternateUrl, CanonicalGroup, CanonicalUrl, DuplicateSurfaceHint, HreflangTag};

    #[test]
    fn validates_canonical_urls() {
        assert!(CanonicalUrl::new("https://example.com/").is_ok());
        assert!(CanonicalUrl::new("/relative").is_err());
    }

    #[test]
    fn normalizes_hreflang_tags() {
        assert_eq!(HreflangTag::new("EN-us").unwrap().as_str(), "en-US");
        assert_eq!(HreflangTag::new("x-default").unwrap().as_str(), "x-default");
        assert!(HreflangTag::new("too-many-parts").is_err());
    }

    #[test]
    fn builds_canonical_groups() {
        let group = CanonicalGroup::new(CanonicalUrl::new("https://example.com/en/").unwrap())
            .with_alternate(
                AlternateUrl::new("https://example.com/es/")
                    .unwrap()
                    .with_hreflang(HreflangTag::new("es").unwrap()),
            )
            .with_hint(DuplicateSurfaceHint::LocaleVariant);

        assert_eq!(group.alternates().len(), 1);
        assert_eq!(group.hints(), &[DuplicateSurfaceHint::LocaleVariant]);
    }
}
