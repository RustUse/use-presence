#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{
    fmt::{self, Write},
    str::FromStr,
};
use std::error::Error;

fn is_http_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://")) && value.contains('.')
}

fn validate_url(value: impl AsRef<str>) -> Result<String, SitemapValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        return Err(SitemapValueError::EmptyUrl);
    }
    if is_http_url(trimmed) {
        Ok(trimmed.to_string())
    } else {
        Err(SitemapValueError::InvalidUrl)
    }
}

/// Error returned by sitemap primitive constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SitemapValueError {
    /// The URL was empty after trimming whitespace.
    EmptyUrl,
    /// The URL did not look like an HTTP or HTTPS URL.
    InvalidUrl,
    /// Priority was outside `0.0..=1.0` or not finite.
    InvalidPriority(f32),
    /// The last-modified label was empty.
    EmptyLastModified,
}

impl fmt::Display for SitemapValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUrl => formatter.write_str("sitemap URL cannot be empty"),
            Self::InvalidUrl => {
                formatter.write_str("sitemap URL must start with http:// or https://")
            },
            Self::InvalidPriority(value) => write!(formatter, "invalid sitemap priority {value}"),
            Self::EmptyLastModified => formatter.write_str("last-modified label cannot be empty"),
        }
    }
}

impl Error for SitemapValueError {}

/// A validated sitemap URL.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SitemapUrl(String);

impl SitemapUrl {
    /// Creates a sitemap URL from an HTTP or HTTPS URL-like string.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapValueError`] when the URL is empty or unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SitemapValueError> {
        validate_url(value).map(Self)
    }

    /// Returns the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SitemapUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SitemapUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SitemapUrl {
    type Err = SitemapValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Sitemap change frequency label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChangeFrequency {
    /// Always changes.
    Always,
    /// Hourly changes.
    Hourly,
    /// Daily changes.
    Daily,
    /// Weekly changes.
    Weekly,
    /// Monthly changes.
    Monthly,
    /// Yearly changes.
    Yearly,
    /// Never changes.
    Never,
}

impl ChangeFrequency {
    /// Returns the sitemap label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

/// Sitemap priority value in the inclusive `0.0..=1.0` range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Priority(f32);

impl Priority {
    /// Creates a sitemap priority.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapValueError::InvalidPriority`] when the value is outside `0.0..=1.0`.
    pub fn new(value: f32) -> Result<Self, SitemapValueError> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(SitemapValueError::InvalidPriority(value))
        }
    }

    /// Returns the raw priority value.
    #[must_use]
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.1}", self.0)
    }
}

/// Last-modified label for sitemap entries.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LastModified(String);

impl LastModified {
    /// Creates a last-modified label.
    ///
    /// # Errors
    ///
    /// Returns [`SitemapValueError::EmptyLastModified`] when the value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SitemapValueError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(SitemapValueError::EmptyLastModified)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the last-modified label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for LastModified {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A sitemap URL entry.
#[derive(Clone, Debug, PartialEq)]
pub struct SitemapEntry {
    url: SitemapUrl,
    last_modified: Option<LastModified>,
    change_frequency: Option<ChangeFrequency>,
    priority: Option<Priority>,
}

impl SitemapEntry {
    /// Creates a sitemap entry from a URL.
    #[must_use]
    pub const fn new(url: SitemapUrl) -> Self {
        Self {
            url,
            last_modified: None,
            change_frequency: None,
            priority: None,
        }
    }

    /// Sets the last-modified label.
    #[must_use]
    pub fn with_last_modified(mut self, last_modified: LastModified) -> Self {
        self.last_modified = Some(last_modified);
        self
    }

    /// Sets the change-frequency label.
    #[must_use]
    pub const fn with_change_frequency(mut self, frequency: ChangeFrequency) -> Self {
        self.change_frequency = Some(frequency);
        self
    }

    /// Sets the priority value.
    #[must_use]
    pub const fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Returns the entry URL.
    #[must_use]
    pub const fn url(&self) -> &SitemapUrl {
        &self.url
    }

    /// Formats this entry as a compact XML `<url>` block.
    #[must_use]
    pub fn to_url_xml(&self) -> String {
        let mut xml = format!("<url><loc>{}</loc>", escape_xml(self.url.as_str()));

        if let Some(last_modified) = &self.last_modified {
            let _ = write!(
                xml,
                "<lastmod>{}</lastmod>",
                escape_xml(last_modified.as_str())
            );
        }
        if let Some(frequency) = self.change_frequency {
            let _ = write!(xml, "<changefreq>{}</changefreq>", frequency.as_str());
        }
        if let Some(priority) = self.priority {
            let _ = write!(xml, "<priority>{priority}</priority>");
        }

        xml.push_str("</url>");
        xml
    }
}

/// A sitemap index entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SitemapIndexEntry {
    sitemap: SitemapUrl,
    last_modified: Option<LastModified>,
}

impl SitemapIndexEntry {
    /// Creates a sitemap index entry.
    #[must_use]
    pub const fn new(sitemap: SitemapUrl) -> Self {
        Self {
            sitemap,
            last_modified: None,
        }
    }

    /// Sets the last-modified label.
    #[must_use]
    pub fn with_last_modified(mut self, last_modified: LastModified) -> Self {
        self.last_modified = Some(last_modified);
        self
    }

    /// Formats this entry as a compact XML `<sitemap>` block.
    #[must_use]
    pub fn to_index_xml(&self) -> String {
        let mut xml = format!("<sitemap><loc>{}</loc>", escape_xml(self.sitemap.as_str()));
        if let Some(last_modified) = &self.last_modified {
            let _ = write!(
                xml,
                "<lastmod>{}</lastmod>",
                escape_xml(last_modified.as_str())
            );
        }
        xml.push_str("</sitemap>");
        xml
    }
}

/// Escapes the XML characters required by sitemap formatting helpers.
#[must_use]
pub fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{
        ChangeFrequency, LastModified, Priority, SitemapEntry, SitemapIndexEntry, SitemapUrl,
        escape_xml,
    };

    #[test]
    fn validates_urls_and_priorities() {
        assert!(SitemapUrl::new("https://example.com/").is_ok());
        assert!(SitemapUrl::new("ftp://example.com/").is_err());
        assert!(Priority::new(1.2).is_err());
    }

    #[test]
    fn formats_url_xml() {
        let entry = SitemapEntry::new(SitemapUrl::new("https://example.com/?a=1&b=2").unwrap())
            .with_last_modified(LastModified::new("2026-05-19").unwrap())
            .with_change_frequency(ChangeFrequency::Weekly)
            .with_priority(Priority::new(0.8).unwrap());

        assert!(entry.to_url_xml().contains("a=1&amp;b=2"));
        assert!(entry.to_url_xml().contains("<priority>0.8</priority>"));
    }

    #[test]
    fn formats_index_xml() {
        let entry =
            SitemapIndexEntry::new(SitemapUrl::new("https://example.com/sitemap.xml").unwrap());

        assert!(entry.to_index_xml().contains("<sitemap>"));
        assert_eq!(escape_xml("A&B"), "A&amp;B");
    }
}
