#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_label(
    value: impl AsRef<str>,
    field: &'static str,
) -> Result<String, AttributionValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AttributionValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by attribution primitive constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributionValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// Attribution window days must be greater than zero.
    InvalidWindow(u16),
    /// Attribution credit must be in the inclusive `0.0..=1.0` range.
    InvalidCredit(f32),
}

impl fmt::Display for AttributionValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidWindow(days) => write!(formatter, "invalid attribution window {days}"),
            Self::InvalidCredit(value) => write!(formatter, "invalid attribution credit {value}"),
        }
    }
}

impl Error for AttributionValueError {}

macro_rules! attribution_label {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates an attribution label.
            ///
            /// # Errors
            ///
            /// Returns [`AttributionValueError::Empty`] when the label is empty.
            pub fn new(value: impl AsRef<str>) -> Result<Self, AttributionValueError> {
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
            type Err = AttributionValueError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

attribution_label!(AttributionSource, "attribution source");
attribution_label!(AttributionMedium, "attribution medium");
attribution_label!(ConversionLabel, "conversion label");

/// Attribution window represented in days.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttributionWindow {
    days: u16,
}

impl AttributionWindow {
    /// Creates an attribution window.
    ///
    /// # Errors
    ///
    /// Returns [`AttributionValueError::InvalidWindow`] when days is zero.
    pub const fn from_days(days: u16) -> Result<Self, AttributionValueError> {
        if days == 0 {
            Err(AttributionValueError::InvalidWindow(days))
        } else {
            Ok(Self { days })
        }
    }

    /// Returns the window length in days.
    #[must_use]
    pub const fn days(self) -> u16 {
        self.days
    }
}

/// Attribution credit represented as a `0.0..=1.0` share.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttributionCredit {
    value: f32,
}

impl AttributionCredit {
    /// Creates an attribution credit value.
    ///
    /// # Errors
    ///
    /// Returns [`AttributionValueError::InvalidCredit`] when the value is outside `0.0..=1.0`.
    pub fn new(value: f32) -> Result<Self, AttributionValueError> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self { value })
        } else {
            Err(AttributionValueError::InvalidCredit(value))
        }
    }

    /// Returns the credit share.
    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }
}

/// Attribution model kind label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttributionModelKind {
    /// First-touch model label.
    FirstTouch,
    /// Last-touch model label.
    LastTouch,
    /// Linear model label.
    Linear,
    /// Time-decay model label.
    TimeDecay,
    /// Position-based model label.
    PositionBased,
    /// Custom model label.
    Custom(String),
}

impl AttributionModelKind {
    /// Creates a custom model label.
    ///
    /// # Errors
    ///
    /// Returns [`AttributionValueError::Empty`] when the label is empty.
    pub fn custom(value: impl AsRef<str>) -> Result<Self, AttributionValueError> {
        validate_label(value, "attribution model").map(Self::Custom)
    }

    /// Returns the model kind label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::FirstTouch => "first-touch",
            Self::LastTouch => "last-touch",
            Self::Linear => "linear",
            Self::TimeDecay => "time-decay",
            Self::PositionBased => "position-based",
            Self::Custom(value) => value,
        }
    }
}

/// Attribution touchpoint primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct Touchpoint {
    source: AttributionSource,
    medium: AttributionMedium,
    conversion_label: Option<ConversionLabel>,
    window: Option<AttributionWindow>,
    credit: Option<AttributionCredit>,
    model_kind: Option<AttributionModelKind>,
}

impl Touchpoint {
    /// Creates an attribution touchpoint.
    #[must_use]
    pub const fn new(source: AttributionSource, medium: AttributionMedium) -> Self {
        Self {
            source,
            medium,
            conversion_label: None,
            window: None,
            credit: None,
            model_kind: None,
        }
    }

    /// Sets the conversion label.
    #[must_use]
    pub fn with_conversion_label(mut self, label: ConversionLabel) -> Self {
        self.conversion_label = Some(label);
        self
    }

    /// Sets the attribution window.
    #[must_use]
    pub const fn with_window(mut self, window: AttributionWindow) -> Self {
        self.window = Some(window);
        self
    }

    /// Sets the attribution credit.
    #[must_use]
    pub const fn with_credit(mut self, credit: AttributionCredit) -> Self {
        self.credit = Some(credit);
        self
    }

    /// Sets the model-kind label.
    #[must_use]
    pub fn with_model_kind(mut self, model_kind: AttributionModelKind) -> Self {
        self.model_kind = Some(model_kind);
        self
    }

    /// Returns the attribution source.
    #[must_use]
    pub const fn source(&self) -> &AttributionSource {
        &self.source
    }

    /// Returns the attribution medium.
    #[must_use]
    pub const fn medium(&self) -> &AttributionMedium {
        &self.medium
    }

    /// Returns attribution credit.
    #[must_use]
    pub const fn credit(&self) -> Option<AttributionCredit> {
        self.credit
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AttributionCredit, AttributionMedium, AttributionModelKind, AttributionSource,
        AttributionWindow, ConversionLabel, Touchpoint,
    };

    #[test]
    fn validates_labels_windows_and_credit() {
        assert!(AttributionSource::new("newsletter").is_ok());
        assert!(AttributionWindow::from_days(0).is_err());
        assert!(AttributionCredit::new(1.5).is_err());
    }

    #[test]
    fn composes_touchpoints() {
        let touchpoint = Touchpoint::new(
            AttributionSource::new("newsletter").unwrap(),
            AttributionMedium::new("email").unwrap(),
        )
        .with_conversion_label(ConversionLabel::new("signup").unwrap())
        .with_window(AttributionWindow::from_days(30).unwrap())
        .with_credit(AttributionCredit::new(0.5).unwrap())
        .with_model_kind(AttributionModelKind::LastTouch);

        assert_eq!(touchpoint.source().as_str(), "newsletter");
        assert!((touchpoint.credit().unwrap().value() - 0.5).abs() < f32::EPSILON);
        assert_eq!(
            AttributionModelKind::custom("weighted").unwrap().as_str(),
            "weighted"
        );
    }
}
