#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_text(
    value: impl AsRef<str>,
    field: &'static str,
    max_length: usize,
) -> Result<String, SeoValueError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        return Err(SeoValueError::Empty { field });
    }

    let actual_length = trimmed.chars().count();
    if actual_length > max_length {
        return Err(SeoValueError::TooLong {
            field,
            max_length,
            actual_length,
        });
    }

    Ok(trimmed.to_string())
}

/// Error returned by SEO primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeoValueError {
    /// The supplied value was empty after trimming whitespace.
    Empty { field: &'static str },
    /// The supplied value exceeded the crate's conservative length hint.
    TooLong {
        field: &'static str,
        max_length: usize,
        actual_length: usize,
    },
    /// The supplied slug hint contained unsupported characters.
    InvalidSlugHint,
}

impl fmt::Display for SeoValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { field } => write!(formatter, "{field} cannot be empty"),
            Self::TooLong {
                field,
                max_length,
                actual_length,
            } => write!(
                formatter,
                "{field} is {actual_length} characters; maximum is {max_length}"
            ),
            Self::InvalidSlugHint => {
                formatter.write_str("slug hint must be ASCII words separated by hyphens")
            },
        }
    }
}

impl Error for SeoValueError {}

/// A validated SEO title label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeoTitle(String);

impl SeoTitle {
    /// Conservative title length hint.
    pub const MAX_LENGTH: usize = 70;

    /// Creates a title from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SeoValueError`] when the title is empty or longer than [`Self::MAX_LENGTH`].
    pub fn new(value: impl AsRef<str>) -> Result<Self, SeoValueError> {
        validate_text(value, "seo title", Self::MAX_LENGTH).map(Self)
    }

    /// Returns the title text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SeoTitle {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SeoTitle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SeoTitle {
    type Err = SeoValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A validated meta description label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaDescription(String);

impl MetaDescription {
    /// Conservative description length hint.
    pub const MAX_LENGTH: usize = 180;

    /// Creates a meta description from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SeoValueError`] when the description is empty or longer than [`Self::MAX_LENGTH`].
    pub fn new(value: impl AsRef<str>) -> Result<Self, SeoValueError> {
        validate_text(value, "meta description", Self::MAX_LENGTH).map(Self)
    }

    /// Returns the description text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MetaDescription {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MetaDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MetaDescription {
    type Err = SeoValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A normalized slug hint for a page or entity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SlugHint(String);

impl SlugHint {
    /// Creates a slug hint from lowercase ASCII words separated by hyphens.
    ///
    /// # Errors
    ///
    /// Returns [`SeoValueError`] when the slug is empty or contains unsupported characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SeoValueError> {
        let trimmed = value.as_ref().trim().trim_matches('/');
        if trimmed.is_empty() {
            return Err(SeoValueError::Empty { field: "slug hint" });
        }

        let normalized = trimmed
            .split(|character: char| character.is_ascii_whitespace() || character == '_')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join("-")
            .to_ascii_lowercase();

        let valid = normalized.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'/')
        }) && !normalized.contains("--");

        if valid {
            Ok(Self(normalized))
        } else {
            Err(SeoValueError::InvalidSlugHint)
        }
    }

    /// Returns the normalized slug hint.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SlugHint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SlugHint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SlugHint {
    type Err = SeoValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Search indexing intent for a page.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndexingHint {
    /// The page may be indexed.
    Index,
    /// The page should not be indexed.
    NoIndex,
    /// Images on the page should not be indexed.
    NoImageIndex,
    /// Snippets should not be shown.
    NoSnippet,
}

impl IndexingHint {
    /// Returns the directive label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Index => "index",
            Self::NoIndex => "noindex",
            Self::NoImageIndex => "noimageindex",
            Self::NoSnippet => "nosnippet",
        }
    }
}

/// Link relation hints used by external surfaces.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LinkRelationHint {
    /// Canonical relationship.
    Canonical,
    /// Alternate surface relationship.
    Alternate,
    /// Previous page relationship.
    Prev,
    /// Next page relationship.
    Next,
    /// Untrusted or unendorsed link relationship.
    NoFollow,
    /// Sponsored link relationship.
    Sponsored,
    /// User-generated content relationship.
    Ugc,
}

impl LinkRelationHint {
    /// Returns the relation label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Alternate => "alternate",
            Self::Prev => "prev",
            Self::Next => "next",
            Self::NoFollow => "nofollow",
            Self::Sponsored => "sponsored",
            Self::Ugc => "ugc",
        }
    }
}

/// Page intent labels for search snippets and metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PageIntent {
    /// Informational page intent.
    Informational,
    /// Navigational page intent.
    Navigational,
    /// Transactional page intent.
    Transactional,
    /// Commercial investigation intent.
    Commercial,
    /// Local discovery intent.
    Local,
    /// Unknown or unspecified page intent.
    Unknown,
}

impl PageIntent {
    /// Returns the intent label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Navigational => "navigational",
            Self::Transactional => "transactional",
            Self::Commercial => "commercial",
            Self::Local => "local",
            Self::Unknown => "unknown",
        }
    }
}

/// Search snippet metadata composed from focused SEO primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchSnippetMetadata {
    title: SeoTitle,
    description: MetaDescription,
    intent: PageIntent,
    indexing_hint: IndexingHint,
    slug_hint: Option<SlugHint>,
    relation_hints: Vec<LinkRelationHint>,
}

impl SearchSnippetMetadata {
    /// Creates snippet metadata with default informational indexable intent.
    #[must_use]
    pub const fn new(title: SeoTitle, description: MetaDescription) -> Self {
        Self {
            title,
            description,
            intent: PageIntent::Informational,
            indexing_hint: IndexingHint::Index,
            slug_hint: None,
            relation_hints: Vec::new(),
        }
    }

    /// Sets the page intent.
    #[must_use]
    pub const fn with_intent(mut self, intent: PageIntent) -> Self {
        self.intent = intent;
        self
    }

    /// Sets the indexing hint.
    #[must_use]
    pub const fn with_indexing_hint(mut self, hint: IndexingHint) -> Self {
        self.indexing_hint = hint;
        self
    }

    /// Sets the slug hint.
    #[must_use]
    pub fn with_slug_hint(mut self, hint: SlugHint) -> Self {
        self.slug_hint = Some(hint);
        self
    }

    /// Adds a link relation hint.
    #[must_use]
    pub fn with_relation_hint(mut self, hint: LinkRelationHint) -> Self {
        self.relation_hints.push(hint);
        self
    }

    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &SeoTitle {
        &self.title
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &MetaDescription {
        &self.description
    }

    /// Returns the page intent.
    #[must_use]
    pub const fn intent(&self) -> PageIntent {
        self.intent
    }

    /// Returns the indexing hint.
    #[must_use]
    pub const fn indexing_hint(&self) -> IndexingHint {
        self.indexing_hint
    }

    /// Returns the slug hint when present.
    #[must_use]
    pub const fn slug_hint(&self) -> Option<&SlugHint> {
        self.slug_hint.as_ref()
    }

    /// Returns link relation hints.
    #[must_use]
    pub fn relation_hints(&self) -> &[LinkRelationHint] {
        &self.relation_hints
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IndexingHint, LinkRelationHint, MetaDescription, PageIntent, SearchSnippetMetadata,
        SeoTitle, SeoValueError, SlugHint,
    };

    #[test]
    fn validates_title_and_description_lengths() {
        assert!(SeoTitle::new("Example").is_ok());
        assert_eq!(
            SeoTitle::new(" "),
            Err(SeoValueError::Empty { field: "seo title" })
        );
        assert!(MetaDescription::new("a".repeat(MetaDescription::MAX_LENGTH + 1)).is_err());
    }

    #[test]
    fn normalizes_slug_hints() {
        let slug = SlugHint::new(" Local Services ").unwrap();

        assert_eq!(slug.as_str(), "local-services");
        assert!(SlugHint::new("bad?slug").is_err());
    }

    #[test]
    fn composes_search_snippet_metadata() {
        let snippet = SearchSnippetMetadata::new(
            SeoTitle::new("Example Services").unwrap(),
            MetaDescription::new("Service details for Example.").unwrap(),
        )
        .with_intent(PageIntent::Local)
        .with_indexing_hint(IndexingHint::NoIndex)
        .with_slug_hint(SlugHint::new("services").unwrap())
        .with_relation_hint(LinkRelationHint::Canonical);

        assert_eq!(snippet.intent(), PageIntent::Local);
        assert_eq!(snippet.indexing_hint().as_str(), "noindex");
        assert_eq!(snippet.slug_hint().unwrap().as_str(), "services");
        assert_eq!(snippet.relation_hints(), &[LinkRelationHint::Canonical]);
    }
}
