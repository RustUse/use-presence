#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty(value: impl AsRef<str>, field: &'static str) -> Result<String, ListingValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ListingValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

fn is_http_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://")) && value.contains('.')
}

/// Error returned by listing primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListingValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// A URL did not look like an HTTP or HTTPS URL.
    InvalidUrl,
}

impl fmt::Display for ListingValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidUrl => {
                formatter.write_str("listing URL must start with http:// or https://")
            },
        }
    }
}

impl Error for ListingValueError {}

/// A business or directory listing name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListingName(String);

impl ListingName {
    /// Creates a listing name.
    ///
    /// # Errors
    ///
    /// Returns [`ListingValueError::Empty`] when the name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ListingValueError> {
        non_empty(value, "listing name").map(Self)
    }

    /// Returns the listing name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ListingName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ListingName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ListingName {
    type Err = ListingValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A listing profile URL.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListingUrl(String);

impl ListingUrl {
    /// Creates a listing URL from an HTTP or HTTPS URL-like string.
    ///
    /// # Errors
    ///
    /// Returns [`ListingValueError::InvalidUrl`] when the URL shape is unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ListingValueError> {
        let trimmed = non_empty(value, "listing URL")?;
        if is_http_url(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(ListingValueError::InvalidUrl)
        }
    }

    /// Returns the URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ListingUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ListingUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A directory, listing, or citation provider label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListingProvider(String);

impl ListingProvider {
    /// Creates a provider label.
    ///
    /// # Errors
    ///
    /// Returns [`ListingValueError::Empty`] when the provider is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ListingValueError> {
        non_empty(value, "listing provider").map(Self)
    }

    /// Returns the provider label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ListingProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ListingProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A name/address/phone record used for consistency checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NapRecord {
    name: String,
    address: String,
    phone: String,
}

impl NapRecord {
    /// Creates a name/address/phone record.
    ///
    /// # Errors
    ///
    /// Returns [`ListingValueError::Empty`] when any component is empty.
    pub fn new(
        name: impl AsRef<str>,
        address: impl AsRef<str>,
        phone: impl AsRef<str>,
    ) -> Result<Self, ListingValueError> {
        Ok(Self {
            name: non_empty(name, "name")?,
            address: non_empty(address, "address")?,
            phone: non_empty(phone, "phone")?,
        })
    }

    /// Returns the name component.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the address component.
    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns the phone component.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }
}

/// Consistency result for name/address/phone fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NapConsistency {
    pub matches_name: bool,
    pub matches_address: bool,
    pub matches_phone: bool,
}

impl NapConsistency {
    /// Compares two name/address/phone records with case-insensitive text matching.
    #[must_use]
    pub fn compare(expected: &NapRecord, observed: &NapRecord) -> Self {
        Self {
            matches_name: expected.name.eq_ignore_ascii_case(&observed.name),
            matches_address: expected.address.eq_ignore_ascii_case(&observed.address),
            matches_phone: expected.phone == observed.phone,
        }
    }

    /// Returns `true` when all fields match.
    #[must_use]
    pub const fn is_consistent(self) -> bool {
        self.matches_name && self.matches_address && self.matches_phone
    }

    /// Returns a simple `0.0..=1.0` consistency score.
    #[must_use]
    pub fn score(self) -> f32 {
        let matches = u8::from(self.matches_name)
            + u8::from(self.matches_address)
            + u8::from(self.matches_phone);
        f32::from(matches) / 3.0
    }
}

/// Status label for a listing profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ListingStatus {
    /// The listing is claimed or verified.
    Claimed,
    /// The listing is not claimed.
    Unclaimed,
    /// The listing is pending verification or publication.
    Pending,
    /// The listing is suppressed or hidden.
    Suppressed,
    /// The listing is a duplicate.
    Duplicate,
    /// The status is unknown.
    Unknown,
}

/// A citation mention for a listing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Citation {
    name: ListingName,
    provider: ListingProvider,
    url: Option<ListingUrl>,
}

impl Citation {
    /// Creates a citation from a name and provider.
    #[must_use]
    pub const fn new(name: ListingName, provider: ListingProvider) -> Self {
        Self {
            name,
            provider,
            url: None,
        }
    }

    /// Sets the citation URL.
    #[must_use]
    pub fn with_url(mut self, url: ListingUrl) -> Self {
        self.url = Some(url);
        self
    }

    /// Returns the provider.
    #[must_use]
    pub const fn provider(&self) -> &ListingProvider {
        &self.provider
    }
}

/// A business listing profile descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListingProfile {
    name: ListingName,
    provider: ListingProvider,
    status: ListingStatus,
    url: Option<ListingUrl>,
    citation: Option<Citation>,
    nap_record: Option<NapRecord>,
}

impl ListingProfile {
    /// Creates a listing profile.
    #[must_use]
    pub const fn new(name: ListingName, provider: ListingProvider) -> Self {
        Self {
            name,
            provider,
            status: ListingStatus::Unknown,
            url: None,
            citation: None,
            nap_record: None,
        }
    }

    /// Sets the listing status.
    #[must_use]
    pub const fn with_status(mut self, status: ListingStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the profile URL.
    #[must_use]
    pub fn with_url(mut self, url: ListingUrl) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets the citation.
    #[must_use]
    pub fn with_citation(mut self, citation: Citation) -> Self {
        self.citation = Some(citation);
        self
    }

    /// Sets the name/address/phone record.
    #[must_use]
    pub fn with_nap_record(mut self, record: NapRecord) -> Self {
        self.nap_record = Some(record);
        self
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> ListingStatus {
        self.status
    }

    /// Returns the listing name.
    #[must_use]
    pub const fn name(&self) -> &ListingName {
        &self.name
    }

    /// Returns the provider.
    #[must_use]
    pub const fn provider(&self) -> &ListingProvider {
        &self.provider
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Citation, ListingName, ListingProfile, ListingProvider, ListingStatus, ListingUrl,
        NapConsistency, NapRecord,
    };

    #[test]
    fn validates_listing_url_shape() {
        assert!(ListingUrl::new("https://example.com/listing").is_ok());
        assert!(ListingUrl::new("example.com/listing").is_err());
    }

    #[test]
    fn scores_nap_consistency() {
        let expected = NapRecord::new("Example Cafe", "1 Main St", "+1-555").unwrap();
        let observed = NapRecord::new("example cafe", "1 Main St", "+1-000").unwrap();
        let consistency = NapConsistency::compare(&expected, &observed);

        assert!(!consistency.is_consistent());
        assert!((consistency.score() - (2.0 / 3.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn builds_listing_profile() {
        let name = ListingName::new("Example Cafe").unwrap();
        let provider = ListingProvider::new("Directory").unwrap();
        let citation = Citation::new(name.clone(), provider.clone());
        let profile = ListingProfile::new(name, provider)
            .with_status(ListingStatus::Claimed)
            .with_citation(citation);

        assert_eq!(profile.status(), ListingStatus::Claimed);
        assert_eq!(profile.provider().as_str(), "Directory");
    }
}
