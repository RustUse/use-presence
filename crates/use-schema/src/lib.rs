#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

fn non_empty(value: impl AsRef<str>, field: &'static str) -> Result<String, SchemaValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(SchemaValueError::Empty { field })
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by structured-data primitive constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SchemaValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// A collection required at least one item.
    EmptyCollection { field: &'static str },
    /// Rating value was outside the inclusive `0.0..=5.0` range or not finite.
    InvalidRating(f32),
}

impl fmt::Display for SchemaValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::EmptyCollection { field } => {
                write!(formatter, "{field} must contain at least one item")
            },
            Self::InvalidRating(value) => write!(formatter, "invalid aggregate rating {value}"),
        }
    }
}

impl Error for SchemaValueError {}

/// Postal address primitive for structured-data records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostalAddress {
    street_address: String,
    address_locality: String,
    address_region: String,
    postal_code: String,
    address_country: String,
}

impl PostalAddress {
    /// Creates a postal address record.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when any address component is empty.
    pub fn new(
        street_address: impl AsRef<str>,
        address_locality: impl AsRef<str>,
        address_region: impl AsRef<str>,
        postal_code: impl AsRef<str>,
        address_country: impl AsRef<str>,
    ) -> Result<Self, SchemaValueError> {
        Ok(Self {
            street_address: non_empty(street_address, "street address")?,
            address_locality: non_empty(address_locality, "address locality")?,
            address_region: non_empty(address_region, "address region")?,
            postal_code: non_empty(postal_code, "postal code")?,
            address_country: non_empty(address_country, "address country")?,
        })
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "PostalAddress"
    }

    /// Returns the street address.
    #[must_use]
    pub fn street_address(&self) -> &str {
        &self.street_address
    }
}

/// Aggregate rating primitive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AggregateRating {
    rating_value: f32,
    review_count: u32,
}

impl AggregateRating {
    /// Creates an aggregate rating.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::InvalidRating`] when the value is outside `0.0..=5.0`.
    pub fn new(rating_value: f32, review_count: u32) -> Result<Self, SchemaValueError> {
        if rating_value.is_finite() && (0.0..=5.0).contains(&rating_value) {
            Ok(Self {
                rating_value,
                review_count,
            })
        } else {
            Err(SchemaValueError::InvalidRating(rating_value))
        }
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "AggregateRating"
    }

    /// Returns the rating value.
    #[must_use]
    pub const fn rating_value(self) -> f32 {
        self.rating_value
    }

    /// Returns the review count.
    #[must_use]
    pub const fn review_count(self) -> u32 {
        self.review_count
    }
}

/// Opening-hours specification primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpeningHoursSpecification {
    day_of_week: String,
    opens: String,
    closes: String,
}

impl OpeningHoursSpecification {
    /// Creates an opening-hours specification.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when any component is empty.
    pub fn new(
        day_of_week: impl AsRef<str>,
        opens: impl AsRef<str>,
        closes: impl AsRef<str>,
    ) -> Result<Self, SchemaValueError> {
        Ok(Self {
            day_of_week: non_empty(day_of_week, "day of week")?,
            opens: non_empty(opens, "opens")?,
            closes: non_empty(closes, "closes")?,
        })
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "OpeningHoursSpecification"
    }
}

/// Organization primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Organization {
    name: String,
    url: Option<String>,
    address: Option<PostalAddress>,
}

impl Organization {
    /// Creates an organization record.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the name is empty.
    pub fn new(name: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        Ok(Self {
            name: non_empty(name, "organization name")?,
            url: None,
            address: None,
        })
    }

    /// Sets a URL label.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the URL label is empty.
    pub fn with_url(mut self, url: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        self.url = Some(non_empty(url, "organization URL")?);
        Ok(self)
    }

    /// Sets a postal address.
    #[must_use]
    pub fn with_address(mut self, address: PostalAddress) -> Self {
        self.address = Some(address);
        self
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "Organization"
    }

    /// Returns the organization name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Local business primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBusiness {
    name: String,
    address: PostalAddress,
    categories: Vec<String>,
    opening_hours: Vec<OpeningHoursSpecification>,
}

impl LocalBusiness {
    /// Creates a local business record.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the name is empty.
    pub fn new(name: impl AsRef<str>, address: PostalAddress) -> Result<Self, SchemaValueError> {
        Ok(Self {
            name: non_empty(name, "local business name")?,
            address,
            categories: Vec::new(),
            opening_hours: Vec::new(),
        })
    }

    /// Adds a category label.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the category is empty.
    pub fn with_category(mut self, category: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        self.categories
            .push(non_empty(category, "local business category")?);
        Ok(self)
    }

    /// Adds opening-hours specification.
    #[must_use]
    pub fn with_opening_hours(mut self, hours: OpeningHoursSpecification) -> Self {
        self.opening_hours.push(hours);
        self
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "LocalBusiness"
    }
}

/// Product primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    name: String,
    sku: Option<String>,
    aggregate_rating: Option<AggregateRating>,
}

impl Product {
    /// Creates a product record.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the name is empty.
    pub fn new(name: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        Ok(Self {
            name: non_empty(name, "product name")?,
            sku: None,
            aggregate_rating: None,
        })
    }

    /// Sets a SKU label.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the SKU is empty.
    pub fn with_sku(mut self, sku: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        self.sku = Some(non_empty(sku, "SKU")?);
        Ok(self)
    }

    /// Sets the aggregate rating.
    #[must_use]
    pub const fn with_aggregate_rating(mut self, rating: AggregateRating) -> Self {
        self.aggregate_rating = Some(rating);
        self
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "Product"
    }
}

/// Article primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Article {
    headline: String,
    author: Option<String>,
}

impl Article {
    /// Creates an article record.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the headline is empty.
    pub fn new(headline: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        Ok(Self {
            headline: non_empty(headline, "article headline")?,
            author: None,
        })
    }

    /// Sets the author label.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the author is empty.
    pub fn with_author(mut self, author: impl AsRef<str>) -> Result<Self, SchemaValueError> {
        self.author = Some(non_empty(author, "article author")?);
        Ok(self)
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "Article"
    }
}

/// Breadcrumb primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Breadcrumb {
    items: Vec<String>,
}

impl Breadcrumb {
    /// Creates a breadcrumb trail from non-empty labels.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError`] when the trail is empty or contains empty labels.
    pub fn new(items: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Self, SchemaValueError> {
        let values = items
            .into_iter()
            .map(|item| non_empty(item, "breadcrumb item"))
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            Err(SchemaValueError::EmptyCollection {
                field: "breadcrumb items",
            })
        } else {
            Ok(Self { items: values })
        }
    }

    /// Returns breadcrumb items.
    #[must_use]
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "BreadcrumbList"
    }
}

/// FAQ entry primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FAQEntry {
    question: String,
    answer: String,
}

impl FAQEntry {
    /// Creates an FAQ entry.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::Empty`] when the question or answer is empty.
    pub fn new(
        question: impl AsRef<str>,
        answer: impl AsRef<str>,
    ) -> Result<Self, SchemaValueError> {
        Ok(Self {
            question: non_empty(question, "FAQ question")?,
            answer: non_empty(answer, "FAQ answer")?,
        })
    }

    /// Returns the question label.
    #[must_use]
    pub fn question(&self) -> &str {
        &self.question
    }
}

/// FAQ structured-data primitive.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FAQ {
    entries: Vec<FAQEntry>,
}

impl FAQ {
    /// Creates an FAQ collection.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaValueError::EmptyCollection`] when no entries are supplied.
    pub fn new(entries: Vec<FAQEntry>) -> Result<Self, SchemaValueError> {
        if entries.is_empty() {
            Err(SchemaValueError::EmptyCollection {
                field: "FAQ entries",
            })
        } else {
            Ok(Self { entries })
        }
    }

    /// Returns entries.
    #[must_use]
    pub fn entries(&self) -> &[FAQEntry] {
        &self.entries
    }

    /// Returns the schema-like type label.
    #[must_use]
    pub const fn schema_type(&self) -> &'static str {
        "FAQPage"
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AggregateRating, Breadcrumb, FAQ, FAQEntry, LocalBusiness, OpeningHoursSpecification,
        Organization, PostalAddress, Product,
    };

    fn address() -> PostalAddress {
        PostalAddress::new("1 Main St", "Portland", "OR", "97201", "US").unwrap()
    }

    #[test]
    fn builds_organization_and_local_business() {
        let organization = Organization::new("Example Co")
            .unwrap()
            .with_address(address());
        let business = LocalBusiness::new("Example Cafe", address())
            .unwrap()
            .with_category("Cafe")
            .unwrap()
            .with_opening_hours(
                OpeningHoursSpecification::new("Monday", "09:00", "17:00").unwrap(),
            );

        assert_eq!(organization.schema_type(), "Organization");
        assert_eq!(business.schema_type(), "LocalBusiness");
    }

    #[test]
    fn validates_rating_and_product() {
        let rating = AggregateRating::new(4.5, 12).unwrap();
        let product = Product::new("Example Product")
            .unwrap()
            .with_sku("SKU-1")
            .unwrap()
            .with_aggregate_rating(rating);

        assert_eq!(product.schema_type(), "Product");
        assert!(AggregateRating::new(6.0, 1).is_err());
    }

    #[test]
    fn builds_breadcrumbs_and_faq() {
        let breadcrumb = Breadcrumb::new(["Home", "Services"]).unwrap();
        let faq = FAQ::new(vec![FAQEntry::new("What?", "A primitive.").unwrap()]).unwrap();

        assert_eq!(breadcrumb.items().len(), 2);
        assert_eq!(faq.schema_type(), "FAQPage");
        assert!(FAQ::new(Vec::new()).is_err());
    }
}
