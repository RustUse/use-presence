#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty(value: impl AsRef<str>, field: &'static str) -> Result<String, LocalValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(LocalValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by local presence constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
}

impl fmt::Display for LocalValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
        }
    }
}

impl Error for LocalValueError {}

/// A business location label or address string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BusinessLocation(String);

impl BusinessLocation {
    /// Creates a business location label.
    ///
    /// # Errors
    ///
    /// Returns [`LocalValueError::Empty`] when the label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, LocalValueError> {
        non_empty(value, "business location").map(Self)
    }

    /// Returns the location label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BusinessLocation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BusinessLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BusinessLocation {
    type Err = LocalValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Human-readable opening-hours label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OpeningHoursLabel(String);

impl OpeningHoursLabel {
    /// Creates an opening-hours label.
    ///
    /// # Errors
    ///
    /// Returns [`LocalValueError::Empty`] when the label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, LocalValueError> {
        non_empty(value, "opening hours label").map(Self)
    }

    /// Returns the label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OpeningHoursLabel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Local category label for external surfaces.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalCategory(String);

impl LocalCategory {
    /// Creates a local category label.
    ///
    /// # Errors
    ///
    /// Returns [`LocalValueError::Empty`] when the category is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, LocalValueError> {
        non_empty(value, "local category").map(Self)
    }

    /// Returns the category label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for LocalCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LocalCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LocalCategory {
    type Err = LocalValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Local business presence shape.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BusinessPresenceKind {
    /// Customers can visit a storefront.
    Storefront,
    /// The business serves customers in an area without public storefront visits.
    ServiceArea,
    /// The business has both storefront and service-area behavior.
    Hybrid,
}

impl BusinessPresenceKind {
    /// Returns `true` when the kind includes service-area behavior.
    #[must_use]
    pub const fn is_service_area(self) -> bool {
        matches!(self, Self::ServiceArea | Self::Hybrid)
    }
}

/// Local visibility hint for external profiles.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LocalVisibilityHint {
    /// Customers should book before arriving.
    AppointmentOnly,
    /// Walk-ins are accepted.
    WalkInsAccepted,
    /// Delivery is available.
    Delivers,
    /// Service happens at a customer location.
    OnSiteService,
    /// Service is available online.
    OnlineService,
    /// Local pickup is available.
    LocalPickup,
}

/// A local service-area business record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceAreaBusiness {
    name: String,
    kind: BusinessPresenceKind,
    location: Option<BusinessLocation>,
    opening_hours: Option<OpeningHoursLabel>,
    service_area_label: Option<String>,
    categories: Vec<LocalCategory>,
    visibility_hints: Vec<LocalVisibilityHint>,
}

impl ServiceAreaBusiness {
    /// Creates a local business descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`LocalValueError::Empty`] when the business name is empty.
    pub fn new(name: impl AsRef<str>, kind: BusinessPresenceKind) -> Result<Self, LocalValueError> {
        Ok(Self {
            name: non_empty(name, "business name")?,
            kind,
            location: None,
            opening_hours: None,
            service_area_label: None,
            categories: Vec::new(),
            visibility_hints: Vec::new(),
        })
    }

    /// Sets a location label.
    #[must_use]
    pub fn with_location(mut self, location: BusinessLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets an opening-hours label.
    #[must_use]
    pub fn with_opening_hours(mut self, label: OpeningHoursLabel) -> Self {
        self.opening_hours = Some(label);
        self
    }

    /// Sets a service-area label.
    ///
    /// # Errors
    ///
    /// Returns [`LocalValueError::Empty`] when the label is empty.
    pub fn with_service_area_label(
        mut self,
        label: impl AsRef<str>,
    ) -> Result<Self, LocalValueError> {
        self.service_area_label = Some(non_empty(label, "service area label")?);
        Ok(self)
    }

    /// Adds a local category.
    #[must_use]
    pub fn with_category(mut self, category: LocalCategory) -> Self {
        self.categories.push(category);
        self
    }

    /// Adds a visibility hint.
    #[must_use]
    pub fn with_visibility_hint(mut self, hint: LocalVisibilityHint) -> Self {
        self.visibility_hints.push(hint);
        self
    }

    /// Returns the business name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the presence kind.
    #[must_use]
    pub const fn kind(&self) -> BusinessPresenceKind {
        self.kind
    }

    /// Returns `true` when the business includes service-area behavior.
    #[must_use]
    pub const fn is_service_area_business(&self) -> bool {
        self.kind.is_service_area()
    }

    /// Returns the optional service-area label.
    #[must_use]
    pub fn service_area_label(&self) -> Option<&str> {
        self.service_area_label.as_deref()
    }

    /// Returns local categories.
    #[must_use]
    pub fn categories(&self) -> &[LocalCategory] {
        &self.categories
    }

    /// Returns visibility hints.
    #[must_use]
    pub fn visibility_hints(&self) -> &[LocalVisibilityHint] {
        &self.visibility_hints
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BusinessLocation, BusinessPresenceKind, LocalCategory, LocalValueError,
        LocalVisibilityHint, OpeningHoursLabel, ServiceAreaBusiness,
    };

    #[test]
    fn validates_local_labels() {
        assert_eq!(
            BusinessLocation::new(" "),
            Err(LocalValueError::Empty {
                field: "business location"
            })
        );
        assert_eq!(
            OpeningHoursLabel::new("Mon-Fri").unwrap().as_str(),
            "Mon-Fri"
        );
    }

    #[test]
    fn composes_service_area_business() {
        let business = ServiceAreaBusiness::new("Example Plumbing", BusinessPresenceKind::Hybrid)
            .unwrap()
            .with_category(LocalCategory::new("Plumber").unwrap())
            .with_visibility_hint(LocalVisibilityHint::OnSiteService)
            .with_service_area_label("Metro area")
            .unwrap();

        assert!(business.is_service_area_business());
        assert_eq!(business.service_area_label(), Some("Metro area"));
        assert_eq!(business.categories().len(), 1);
    }
}
