#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_label(
    value: impl AsRef<str>,
    field: &'static str,
) -> Result<String, CampaignValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(CampaignValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by campaign primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CampaignValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
}

impl fmt::Display for CampaignValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
        }
    }
}

impl Error for CampaignValueError {}

macro_rules! campaign_label {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a campaign label.
            ///
            /// # Errors
            ///
            /// Returns [`CampaignValueError::Empty`] when the label is empty.
            pub fn new(value: impl AsRef<str>) -> Result<Self, CampaignValueError> {
                validate_label(value, $field).map(Self)
            }

            /// Returns the label text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = CampaignValueError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

campaign_label!(CampaignId, "campaign ID");
campaign_label!(CampaignName, "campaign name");
campaign_label!(CampaignMedium, "campaign medium");
campaign_label!(CampaignLabel, "campaign label");

/// Generic campaign channel label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CampaignChannel {
    /// Organic search channel.
    OrganicSearch,
    /// Paid search channel.
    PaidSearch,
    /// Social channel.
    Social,
    /// Email channel.
    Email,
    /// Referral channel.
    Referral,
    /// Direct channel.
    Direct,
    /// Display channel.
    Display,
    /// Other custom channel label.
    Other(String),
}

impl CampaignChannel {
    /// Creates a custom campaign channel.
    ///
    /// # Errors
    ///
    /// Returns [`CampaignValueError::Empty`] when the label is empty.
    pub fn other(value: impl AsRef<str>) -> Result<Self, CampaignValueError> {
        validate_label(value, "campaign channel").map(Self::Other)
    }

    /// Returns the channel label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::OrganicSearch => "organic-search",
            Self::PaidSearch => "paid-search",
            Self::Social => "social",
            Self::Email => "email",
            Self::Referral => "referral",
            Self::Direct => "direct",
            Self::Display => "display",
            Self::Other(value) => value,
        }
    }
}

/// Campaign status label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CampaignStatus {
    /// Draft campaign.
    Draft,
    /// Scheduled campaign.
    Scheduled,
    /// Active campaign.
    Active,
    /// Paused campaign.
    Paused,
    /// Ended campaign.
    Ended,
    /// Archived campaign.
    Archived,
}

impl CampaignStatus {
    /// Returns the status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Scheduled => "scheduled",
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Ended => "ended",
            Self::Archived => "archived",
        }
    }
}

/// Campaign flight labels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignFlight {
    start_label: String,
    end_label: Option<String>,
}

impl CampaignFlight {
    /// Creates a campaign flight with a start label.
    ///
    /// # Errors
    ///
    /// Returns [`CampaignValueError::Empty`] when the start label is empty.
    pub fn new(start_label: impl AsRef<str>) -> Result<Self, CampaignValueError> {
        Ok(Self {
            start_label: validate_label(start_label, "campaign flight start")?,
            end_label: None,
        })
    }

    /// Sets an end label.
    ///
    /// # Errors
    ///
    /// Returns [`CampaignValueError::Empty`] when the end label is empty.
    pub fn with_end_label(
        mut self,
        end_label: impl AsRef<str>,
    ) -> Result<Self, CampaignValueError> {
        self.end_label = Some(validate_label(end_label, "campaign flight end")?);
        Ok(self)
    }

    /// Returns the start label.
    #[must_use]
    pub fn start_label(&self) -> &str {
        &self.start_label
    }

    /// Returns the optional end label.
    #[must_use]
    pub fn end_label(&self) -> Option<&str> {
        self.end_label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CampaignChannel, CampaignFlight, CampaignId, CampaignLabel, CampaignMedium, CampaignName,
        CampaignStatus,
    };

    #[test]
    fn validates_campaign_labels() {
        assert_eq!(
            CampaignId::new("spring-2026").unwrap().as_str(),
            "spring-2026"
        );
        assert!(CampaignName::new(" ").is_err());
        assert_eq!(CampaignMedium::new("email").unwrap().as_str(), "email");
        assert_eq!(CampaignLabel::new("launch").unwrap().as_str(), "launch");
    }

    #[test]
    fn exposes_channel_and_status_labels() {
        assert_eq!(CampaignChannel::Email.as_str(), "email");
        assert_eq!(
            CampaignChannel::other("affiliate").unwrap().as_str(),
            "affiliate"
        );
        assert_eq!(CampaignStatus::Paused.as_str(), "paused");
    }

    #[test]
    fn builds_campaign_flights() {
        let flight = CampaignFlight::new("2026-03-01")
            .unwrap()
            .with_end_label("2026-03-31")
            .unwrap();

        assert_eq!(flight.start_label(), "2026-03-01");
        assert_eq!(flight.end_label(), Some("2026-03-31"));
    }
}
