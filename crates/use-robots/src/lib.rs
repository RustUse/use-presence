#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned by robots primitive constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotsValueError {
    /// Bot name was empty after trimming whitespace.
    EmptyBotName,
    /// Bot name contained unsupported characters.
    InvalidBotName,
}

impl fmt::Display for RobotsValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBotName => formatter.write_str("bot name cannot be empty"),
            Self::InvalidBotName => {
                formatter.write_str("bot name must contain visible ASCII characters")
            },
        }
    }
}

impl Error for RobotsValueError {}

/// Bot name label used with robots directives.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BotName(String);

impl BotName {
    /// Creates a bot name label.
    ///
    /// # Errors
    ///
    /// Returns [`RobotsValueError`] when the bot name is empty or unsupported.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotsValueError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            return Err(RobotsValueError::EmptyBotName);
        }
        if trimmed.bytes().all(|byte| byte.is_ascii_graphic()) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err(RobotsValueError::InvalidBotName)
        }
    }

    /// Returns the bot name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BotName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BotName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BotName {
    type Err = RobotsValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Indexing directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndexDirective {
    /// Allow indexing.
    Index,
    /// Disallow indexing.
    NoIndex,
}

impl IndexDirective {
    /// Returns the directive label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Index => "index",
            Self::NoIndex => "noindex",
        }
    }
}

/// Link-following directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FollowDirective {
    /// Allow following links.
    Follow,
    /// Disallow following links.
    NoFollow,
}

impl FollowDirective {
    /// Returns the directive label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Follow => "follow",
            Self::NoFollow => "nofollow",
        }
    }
}

/// Snippet directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SnippetDirective {
    /// Allow snippets.
    Snippet,
    /// Disallow snippets.
    NoSnippet,
    /// Limit snippets to a maximum character count.
    MaxSnippet(u16),
}

impl SnippetDirective {
    /// Formats the directive for a robots meta content value.
    #[must_use]
    pub fn to_content(self) -> String {
        match self {
            Self::Snippet => "snippet".to_string(),
            Self::NoSnippet => "nosnippet".to_string(),
            Self::MaxSnippet(max) => format!("max-snippet:{max}"),
        }
    }
}

/// Archive directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ArchiveDirective {
    /// Allow cached/archive snippets.
    Archive,
    /// Disallow cached/archive snippets.
    NoArchive,
}

impl ArchiveDirective {
    /// Returns the directive label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::NoArchive => "noarchive",
        }
    }
}

/// A robots directive set suitable for meta content formatting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RobotsDirective {
    index: IndexDirective,
    follow: FollowDirective,
    snippet: Option<SnippetDirective>,
    archive: Option<ArchiveDirective>,
}

impl RobotsDirective {
    /// Creates a robots directive from index and follow directives.
    #[must_use]
    pub const fn new(index: IndexDirective, follow: FollowDirective) -> Self {
        Self {
            index,
            follow,
            snippet: None,
            archive: None,
        }
    }

    /// Common `index,follow` directive.
    #[must_use]
    pub const fn index_follow() -> Self {
        Self::new(IndexDirective::Index, FollowDirective::Follow)
    }

    /// Common `noindex,nofollow` directive.
    #[must_use]
    pub const fn noindex_nofollow() -> Self {
        Self::new(IndexDirective::NoIndex, FollowDirective::NoFollow)
    }

    /// Sets the snippet directive.
    #[must_use]
    pub const fn with_snippet(mut self, snippet: SnippetDirective) -> Self {
        self.snippet = Some(snippet);
        self
    }

    /// Sets the archive directive.
    #[must_use]
    pub const fn with_archive(mut self, archive: ArchiveDirective) -> Self {
        self.archive = Some(archive);
        self
    }

    /// Returns the index directive.
    #[must_use]
    pub const fn index(&self) -> IndexDirective {
        self.index
    }

    /// Returns the follow directive.
    #[must_use]
    pub const fn follow(&self) -> FollowDirective {
        self.follow
    }

    /// Formats this directive as a robots meta content value.
    #[must_use]
    pub fn to_meta_content(&self) -> String {
        let mut parts = vec![
            self.index.as_str().to_string(),
            self.follow.as_str().to_string(),
        ];
        if let Some(snippet) = self.snippet {
            parts.push(snippet.to_content());
        }
        if let Some(archive) = self.archive {
            parts.push(archive.as_str().to_string());
        }
        parts.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArchiveDirective, BotName, FollowDirective, IndexDirective, RobotsDirective,
        RobotsValueError, SnippetDirective,
    };

    #[test]
    fn validates_bot_names() {
        assert_eq!(BotName::new(" "), Err(RobotsValueError::EmptyBotName));
        assert_eq!(BotName::new("Googlebot").unwrap().as_str(), "Googlebot");
    }

    #[test]
    fn formats_common_directives() {
        assert_eq!(
            RobotsDirective::index_follow().to_meta_content(),
            "index,follow"
        );
        assert_eq!(
            RobotsDirective::noindex_nofollow().to_meta_content(),
            "noindex,nofollow"
        );
    }

    #[test]
    fn formats_extended_directives() {
        let directive = RobotsDirective::new(IndexDirective::NoIndex, FollowDirective::Follow)
            .with_snippet(SnippetDirective::MaxSnippet(120))
            .with_archive(ArchiveDirective::NoArchive);

        assert_eq!(
            directive.to_meta_content(),
            "noindex,follow,max-snippet:120,noarchive"
        );
    }
}
