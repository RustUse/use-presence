#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_label(value: impl AsRef<str>, field: &'static str) -> Result<String, GeoValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(GeoValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by geographic presence constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GeoValueError {
    /// A text field was empty after trimming whitespace.
    Empty { field: &'static str },
    /// Latitude was outside the inclusive `-90..=90` range or not finite.
    InvalidLatitude(f64),
    /// Longitude was outside the inclusive `-180..=180` range or not finite.
    InvalidLongitude(f64),
    /// Radius was not positive and finite.
    InvalidRadius(f64),
    /// Region code shape was unsupported.
    InvalidRegionCode,
}

impl fmt::Display for GeoValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::InvalidLatitude(value) => write!(formatter, "invalid latitude {value}"),
            Self::InvalidLongitude(value) => write!(formatter, "invalid longitude {value}"),
            Self::InvalidRadius(value) => write!(formatter, "invalid radius {value}"),
            Self::InvalidRegionCode => {
                formatter.write_str("region code must be 2 to 16 ASCII letters, digits, or hyphens")
            },
        }
    }
}

impl Error for GeoValueError {}

/// A latitude/longitude point used as a presence targeting label.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoPoint {
    latitude: f64,
    longitude: f64,
}

impl GeoPoint {
    /// Creates a geographic point from latitude and longitude values.
    ///
    /// # Errors
    ///
    /// Returns [`GeoValueError`] when either coordinate is outside the supported range.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, GeoValueError> {
        if !latitude.is_finite() || !(-90.0..=90.0).contains(&latitude) {
            return Err(GeoValueError::InvalidLatitude(latitude));
        }
        if !longitude.is_finite() || !(-180.0..=180.0).contains(&longitude) {
            return Err(GeoValueError::InvalidLongitude(longitude));
        }

        Ok(Self {
            latitude,
            longitude,
        })
    }

    /// Returns latitude in decimal degrees.
    #[must_use]
    pub const fn latitude(self) -> f64 {
        self.latitude
    }

    /// Returns longitude in decimal degrees.
    #[must_use]
    pub const fn longitude(self) -> f64 {
        self.longitude
    }
}

/// A compact region code used for discovery or targeting labels.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeoRegionCode(String);

impl GeoRegionCode {
    /// Creates a normalized region code.
    ///
    /// # Errors
    ///
    /// Returns [`GeoValueError::InvalidRegionCode`] when the shape is unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeoValueError> {
        let normalized = value.as_ref().trim().to_ascii_uppercase();
        let valid = (2..=16).contains(&normalized.len())
            && normalized
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-');

        if valid {
            Ok(Self(normalized))
        } else {
            Err(GeoValueError::InvalidRegionCode)
        }
    }

    /// Returns the normalized region code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for GeoRegionCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeoRegionCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeoRegionCode {
    type Err = GeoValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A positive radius around a location, stored in meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocationRadius {
    meters: f64,
}

impl LocationRadius {
    /// Creates a radius from meters.
    ///
    /// # Errors
    ///
    /// Returns [`GeoValueError::InvalidRadius`] when the value is not positive and finite.
    pub fn from_meters(meters: f64) -> Result<Self, GeoValueError> {
        if meters.is_finite() && meters > 0.0 {
            Ok(Self { meters })
        } else {
            Err(GeoValueError::InvalidRadius(meters))
        }
    }

    /// Creates a radius from kilometers.
    ///
    /// # Errors
    ///
    /// Returns [`GeoValueError::InvalidRadius`] when the value is not positive and finite.
    pub fn from_kilometers(kilometers: f64) -> Result<Self, GeoValueError> {
        Self::from_meters(kilometers * 1_000.0)
    }

    /// Returns the radius in meters.
    #[must_use]
    pub const fn meters(self) -> f64 {
        self.meters
    }
}

/// A named service area used for local discovery and targeting.
#[derive(Clone, Debug, PartialEq)]
pub struct ServiceArea {
    label: String,
    regions: Vec<GeoRegionCode>,
    radius: Option<LocationRadius>,
}

impl ServiceArea {
    /// Creates a service-area label.
    ///
    /// # Errors
    ///
    /// Returns [`GeoValueError::Empty`] when the label is empty.
    pub fn new(label: impl AsRef<str>) -> Result<Self, GeoValueError> {
        Ok(Self {
            label: validate_label(label, "service area label")?,
            regions: Vec::new(),
            radius: None,
        })
    }

    /// Adds a region code to the area.
    #[must_use]
    pub fn with_region(mut self, region: GeoRegionCode) -> Self {
        self.regions.push(region);
        self
    }

    /// Sets the area radius.
    #[must_use]
    pub const fn with_radius(mut self, radius: LocationRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Returns the area label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns region codes assigned to the area.
    #[must_use]
    pub fn regions(&self) -> &[GeoRegionCode] {
        &self.regions
    }

    /// Returns the optional radius.
    #[must_use]
    pub const fn radius(&self) -> Option<LocationRadius> {
        self.radius
    }
}

/// Geographic targeting shape for presence and discovery surfaces.
#[derive(Clone, Debug, PartialEq)]
pub enum GeoTarget {
    /// A point target.
    Point(GeoPoint),
    /// A region-code target.
    Region(GeoRegionCode),
    /// A named service area target.
    ServiceArea(ServiceArea),
    /// A radius around a point.
    Radius {
        center: GeoPoint,
        radius: LocationRadius,
    },
}

#[cfg(test)]
mod tests {
    use super::{GeoPoint, GeoRegionCode, GeoTarget, GeoValueError, LocationRadius, ServiceArea};

    #[test]
    fn validates_geo_points() {
        let point = GeoPoint::new(45.0, -122.0).unwrap();

        assert!((point.latitude() - 45.0).abs() < f64::EPSILON);
        assert_eq!(
            GeoPoint::new(100.0, 0.0),
            Err(GeoValueError::InvalidLatitude(100.0))
        );
    }

    #[test]
    fn normalizes_region_codes() {
        let region = GeoRegionCode::new("us-or").unwrap();

        assert_eq!(region.as_str(), "US-OR");
        assert!(GeoRegionCode::new("!").is_err());
    }

    #[test]
    fn composes_service_area_targets() {
        let area = ServiceArea::new("Portland metro")
            .unwrap()
            .with_region(GeoRegionCode::new("us-or").unwrap())
            .with_radius(LocationRadius::from_kilometers(30.0).unwrap());
        let target = GeoTarget::ServiceArea(area.clone());

        assert_eq!(area.regions().len(), 1);
        assert!(matches!(target, GeoTarget::ServiceArea(_)));
    }
}
