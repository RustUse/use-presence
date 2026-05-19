#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_component(
    value: impl AsRef<str>,
    field: &'static str,
) -> Result<String, UtmValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        return Err(UtmValueError::Empty { field });
    }
    if trimmed
        .chars()
        .any(|character| character.is_control() || matches!(character, '&' | '=' | '?' | '#'))
    {
        return Err(UtmValueError::Invalid { field });
    }
    Ok(trimmed.to_string())
}

fn is_http_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://")
}

/// Error returned by UTM primitive constructors and parsers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UtmValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// The supplied value contained unsupported characters.
    Invalid { field: &'static str },
    /// A required UTM field was missing.
    MissingField(&'static str),
    /// The base URL was empty or unsupported.
    InvalidUrl,
}

impl fmt::Display for UtmValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::Invalid { field } => {
                write!(formatter, "{field} contains unsupported query characters")
            },
            Self::MissingField(field) => write!(formatter, "missing required {field}"),
            Self::InvalidUrl => formatter.write_str("UTM URL must start with http:// or https://"),
        }
    }
}

impl Error for UtmValueError {}

macro_rules! utm_component {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a UTM component value.
            ///
            /// # Errors
            ///
            /// Returns [`UtmValueError`] when the value is empty or contains query separators.
            pub fn new(value: impl AsRef<str>) -> Result<Self, UtmValueError> {
                validate_component(value, $field).map(Self)
            }

            /// Returns the component text.
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
            type Err = UtmValueError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

utm_component!(UtmSource, "utm_source");
utm_component!(UtmMedium, "utm_medium");
utm_component!(UtmCampaign, "utm_campaign");
utm_component!(UtmTerm, "utm_term");
utm_component!(UtmContent, "utm_content");

/// UTM parameter set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtmParameters {
    source: UtmSource,
    medium: UtmMedium,
    campaign: UtmCampaign,
    term: Option<UtmTerm>,
    content: Option<UtmContent>,
}

impl UtmParameters {
    /// Creates required UTM parameters.
    #[must_use]
    pub const fn new(source: UtmSource, medium: UtmMedium, campaign: UtmCampaign) -> Self {
        Self {
            source,
            medium,
            campaign,
            term: None,
            content: None,
        }
    }

    /// Sets the optional term parameter.
    #[must_use]
    pub fn with_term(mut self, term: UtmTerm) -> Self {
        self.term = Some(term);
        self
    }

    /// Sets the optional content parameter.
    #[must_use]
    pub fn with_content(mut self, content: UtmContent) -> Self {
        self.content = Some(content);
        self
    }

    /// Returns the source.
    #[must_use]
    pub const fn source(&self) -> &UtmSource {
        &self.source
    }

    /// Returns the medium.
    #[must_use]
    pub const fn medium(&self) -> &UtmMedium {
        &self.medium
    }

    /// Returns the campaign.
    #[must_use]
    pub const fn campaign(&self) -> &UtmCampaign {
        &self.campaign
    }

    /// Formats the parameters as a query string.
    #[must_use]
    pub fn to_query_string(&self) -> String {
        let mut parts = vec![
            format!("utm_source={}", self.source),
            format!("utm_medium={}", self.medium),
            format!("utm_campaign={}", self.campaign),
        ];
        if let Some(term) = &self.term {
            parts.push(format!("utm_term={term}"));
        }
        if let Some(content) = &self.content {
            parts.push(format!("utm_content={content}"));
        }
        parts.join("&")
    }
}

/// Parses UTM parameters from a query string or URL.
///
/// # Errors
///
/// Returns [`UtmValueError`] when required fields are missing or values are invalid.
pub fn parse_utm_parameters(input: &str) -> Result<UtmParameters, UtmValueError> {
    let before_fragment = input.split_once('#').map_or(input, |(before, _)| before);
    let query = before_fragment
        .split_once('?')
        .map_or(before_fragment, |(_, query)| query)
        .strip_prefix('?')
        .unwrap_or(before_fragment);

    let mut source = None;
    let mut medium = None;
    let mut campaign = None;
    let mut term = None;
    let mut content = None;

    for segment in query.split('&').filter(|segment| !segment.is_empty()) {
        let Some((key, value)) = segment.split_once('=') else {
            continue;
        };
        match key {
            "utm_source" => source = Some(UtmSource::new(value)?),
            "utm_medium" => medium = Some(UtmMedium::new(value)?),
            "utm_campaign" => campaign = Some(UtmCampaign::new(value)?),
            "utm_term" => term = Some(UtmTerm::new(value)?),
            "utm_content" => content = Some(UtmContent::new(value)?),
            _ => {},
        }
    }

    let params = UtmParameters::new(
        source.ok_or(UtmValueError::MissingField("utm_source"))?,
        medium.ok_or(UtmValueError::MissingField("utm_medium"))?,
        campaign.ok_or(UtmValueError::MissingField("utm_campaign"))?,
    );

    Ok(match (term, content) {
        (Some(term), Some(content)) => params.with_term(term).with_content(content),
        (Some(term), None) => params.with_term(term),
        (None, Some(content)) => params.with_content(content),
        (None, None) => params,
    })
}

/// A URL with UTM parameters attached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtmUrl {
    base_url: String,
    parameters: UtmParameters,
}

impl UtmUrl {
    /// Creates a UTM URL from a base URL and parameters.
    ///
    /// # Errors
    ///
    /// Returns [`UtmValueError::InvalidUrl`] when the base URL is empty or unsupported.
    pub fn new(
        base_url: impl AsRef<str>,
        parameters: UtmParameters,
    ) -> Result<Self, UtmValueError> {
        let trimmed = base_url.as_ref().trim();
        if trimmed.is_empty() || !is_http_url(trimmed) {
            return Err(UtmValueError::InvalidUrl);
        }
        Ok(Self {
            base_url: trimmed.to_string(),
            parameters,
        })
    }

    /// Returns the base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the UTM parameters.
    #[must_use]
    pub const fn parameters(&self) -> &UtmParameters {
        &self.parameters
    }
}

impl fmt::Display for UtmUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = if self.base_url.contains('?') {
            '&'
        } else {
            '?'
        };
        write!(
            formatter,
            "{}{}{}",
            self.base_url,
            separator,
            self.parameters.to_query_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UtmCampaign, UtmContent, UtmMedium, UtmParameters, UtmSource, UtmTerm, UtmUrl,
        parse_utm_parameters,
    };

    fn params() -> UtmParameters {
        UtmParameters::new(
            UtmSource::new("newsletter").unwrap(),
            UtmMedium::new("email").unwrap(),
            UtmCampaign::new("spring").unwrap(),
        )
    }

    #[test]
    fn validates_utm_components() {
        assert!(UtmSource::new("newsletter").is_ok());
        assert!(UtmMedium::new("bad&value").is_err());
    }

    #[test]
    fn formats_and_parses_query_parameters() {
        let parameters = params()
            .with_term(UtmTerm::new("running shoes").unwrap())
            .with_content(UtmContent::new("hero-link").unwrap());
        let parsed = parse_utm_parameters(&parameters.to_query_string()).unwrap();

        assert_eq!(parsed.source().as_str(), "newsletter");
        assert_eq!(
            parameters.to_query_string(),
            "utm_source=newsletter&utm_medium=email&utm_campaign=spring&utm_term=running shoes&utm_content=hero-link"
        );
    }

    #[test]
    fn formats_utm_urls() {
        let url = UtmUrl::new("https://example.com/pricing", params()).unwrap();

        assert_eq!(
            url.to_string(),
            "https://example.com/pricing?utm_source=newsletter&utm_medium=email&utm_campaign=spring"
        );
    }
}
