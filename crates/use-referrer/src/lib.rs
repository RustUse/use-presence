#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty(value: impl AsRef<str>, field: &'static str) -> Result<String, ReferrerValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ReferrerValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

fn host_from_url(value: &str) -> Option<String> {
    let after_scheme = value.split_once("://")?.1;
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .filter(|part| !part.is_empty())?;
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    let host = host_port
        .strip_prefix('[')
        .and_then(|tail| tail.split_once(']').map(|(host, _)| host))
        .unwrap_or_else(|| {
            host_port
                .split_once(':')
                .map_or(host_port, |(host, _)| host)
        });
    (!host.is_empty()).then(|| host.to_ascii_lowercase())
}

/// Error returned by referrer primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferrerValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// The URL did not contain a recognizable host.
    InvalidUrl,
}

impl fmt::Display for ReferrerValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidUrl => formatter.write_str("referrer URL must include a host"),
        }
    }
}

impl Error for ReferrerValueError {}

/// Referrer URL label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReferrerUrl(String);

impl ReferrerUrl {
    /// Creates a referrer URL label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError`] when the URL is empty or lacks a host.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ReferrerValueError> {
        let value = non_empty(value, "referrer URL")?;
        if host_from_url(&value).is_some() {
            Ok(Self(value))
        } else {
            Err(ReferrerValueError::InvalidUrl)
        }
    }

    /// Returns the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extracts the host from the URL.
    #[must_use]
    pub fn host(&self) -> ReferrerHost {
        ReferrerHost(host_from_url(&self.0).unwrap_or_default())
    }
}

impl AsRef<str> for ReferrerUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for ReferrerUrl {
    type Err = ReferrerValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Referrer host label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReferrerHost(String);

impl ReferrerHost {
    /// Creates a referrer host label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError::Empty`] when the host is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ReferrerValueError> {
        non_empty(value, "referrer host").map(|value| Self(value.to_ascii_lowercase()))
    }

    /// Returns the host label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ReferrerHost {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Referrer classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReferrerKind {
    /// Direct traffic.
    Direct,
    /// Organic search traffic.
    Organic,
    /// Paid traffic.
    Paid,
    /// Social traffic.
    Social,
    /// Email traffic.
    Email,
    /// Referral traffic.
    Referral,
    /// Unknown traffic.
    Unknown,
}

/// Source classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceKind {
    /// Direct source.
    Direct,
    /// Search source.
    Search,
    /// Social source.
    Social,
    /// Email source.
    Email,
    /// Paid source.
    Paid,
    /// Referral source.
    Referral,
    /// Other source.
    Other,
}

/// Direct traffic marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectTraffic;

/// Organic traffic label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganicTraffic {
    source: String,
}

impl OrganicTraffic {
    /// Creates an organic traffic label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError::Empty`] when the source is empty.
    pub fn new(source: impl AsRef<str>) -> Result<Self, ReferrerValueError> {
        Ok(Self {
            source: non_empty(source, "organic source")?,
        })
    }
}

/// Paid traffic label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaidTraffic {
    source: String,
    medium: String,
}

impl PaidTraffic {
    /// Creates a paid traffic label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError::Empty`] when the source or medium is empty.
    pub fn new(
        source: impl AsRef<str>,
        medium: impl AsRef<str>,
    ) -> Result<Self, ReferrerValueError> {
        Ok(Self {
            source: non_empty(source, "paid source")?,
            medium: non_empty(medium, "paid medium")?,
        })
    }
}

/// Social traffic label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SocialTraffic {
    network: String,
}

impl SocialTraffic {
    /// Creates a social traffic label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError::Empty`] when the network is empty.
    pub fn new(network: impl AsRef<str>) -> Result<Self, ReferrerValueError> {
        Ok(Self {
            network: non_empty(network, "social network")?,
        })
    }
}

/// Email traffic label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmailTraffic {
    source: String,
}

impl EmailTraffic {
    /// Creates an email traffic label.
    ///
    /// # Errors
    ///
    /// Returns [`ReferrerValueError::Empty`] when the source is empty.
    pub fn new(source: impl AsRef<str>) -> Result<Self, ReferrerValueError> {
        Ok(Self {
            source: non_empty(source, "email source")?,
        })
    }
}

/// Referral traffic label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferralTraffic {
    host: ReferrerHost,
}

impl ReferralTraffic {
    /// Creates a referral traffic label.
    #[must_use]
    pub const fn new(host: ReferrerHost) -> Self {
        Self { host }
    }

    /// Returns the referral host.
    #[must_use]
    pub const fn host(&self) -> &ReferrerHost {
        &self.host
    }
}

/// Classifies source and medium labels into a referrer kind.
#[must_use]
pub fn classify_source_medium(source: Option<&str>, medium: Option<&str>) -> ReferrerKind {
    let source = source.unwrap_or_default().trim().to_ascii_lowercase();
    let medium = medium.unwrap_or_default().trim().to_ascii_lowercase();

    if source.is_empty() && medium.is_empty() || source == "direct" || medium == "none" {
        ReferrerKind::Direct
    } else if matches!(medium.as_str(), "organic" | "seo") {
        ReferrerKind::Organic
    } else if matches!(
        medium.as_str(),
        "cpc" | "ppc" | "paid" | "paid-search" | "display" | "cpm"
    ) {
        ReferrerKind::Paid
    } else if matches!(
        medium.as_str(),
        "social" | "social-media" | "social-network"
    ) {
        ReferrerKind::Social
    } else if matches!(medium.as_str(), "email" | "newsletter") {
        ReferrerKind::Email
    } else if matches!(medium.as_str(), "referral" | "referrer") {
        ReferrerKind::Referral
    } else {
        ReferrerKind::Unknown
    }
}

/// Classifies a source label.
#[must_use]
pub fn classify_source_kind(source: &str) -> SourceKind {
    let source = source.trim().to_ascii_lowercase();
    if source.is_empty() || source == "direct" {
        SourceKind::Direct
    } else if is_search_host(&source) || source.contains("search") {
        SourceKind::Search
    } else if is_social_host(&source) {
        SourceKind::Social
    } else if source.contains("mail") || source.contains("newsletter") {
        SourceKind::Email
    } else {
        SourceKind::Other
    }
}

/// Classifies a referrer host.
#[must_use]
pub fn classify_referrer_host(host: &str) -> ReferrerKind {
    let host = host.trim().to_ascii_lowercase();
    if host.is_empty() {
        ReferrerKind::Direct
    } else if is_search_host(&host) {
        ReferrerKind::Organic
    } else if is_social_host(&host) {
        ReferrerKind::Social
    } else if host.contains("mail") {
        ReferrerKind::Email
    } else {
        ReferrerKind::Referral
    }
}

fn is_search_host(host: &str) -> bool {
    ["google.", "bing.", "duckduckgo.", "yahoo.", "baidu."]
        .iter()
        .any(|needle| host.contains(needle))
}

fn is_social_host(host: &str) -> bool {
    [
        "facebook.",
        "instagram.",
        "linkedin.",
        "twitter.",
        "x.com",
        "t.co",
        "pinterest.",
        "reddit.",
    ]
    .iter()
    .any(|needle| host.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::{
        ReferrerHost, ReferrerKind, ReferrerUrl, classify_referrer_host, classify_source_kind,
        classify_source_medium,
    };

    #[test]
    fn extracts_referrer_hosts() {
        let url = ReferrerUrl::new("https://www.google.com/search?q=rustuse").unwrap();

        assert_eq!(url.host().as_str(), "www.google.com");
        assert!(ReferrerUrl::new("not-a-url").is_err());
    }

    #[test]
    fn classifies_source_and_medium_labels() {
        assert_eq!(classify_source_medium(None, None), ReferrerKind::Direct);
        assert_eq!(
            classify_source_medium(Some("newsletter"), Some("email")),
            ReferrerKind::Email
        );
        assert_eq!(
            classify_source_medium(Some("google"), Some("cpc")),
            ReferrerKind::Paid
        );
    }

    #[test]
    fn classifies_hosts_and_sources() {
        assert_eq!(
            classify_referrer_host("www.google.com"),
            ReferrerKind::Organic
        );
        assert_eq!(classify_referrer_host("facebook.com"), ReferrerKind::Social);
        assert_eq!(classify_source_kind("direct"), super::SourceKind::Direct);
        assert_eq!(
            ReferrerHost::new("Example.com").unwrap().as_str(),
            "example.com"
        );
    }
}
