use std::{io, path::PathBuf};

use thiserror::Error;

/// All errors that can occur during the execution of Convlint.
#[derive(Debug, Error)]
pub enum ModelError {
    /// Convlint was not able to serialize
    /// program structure to TOML.
    #[error("Failed to serialize to TOML: {0}")]
    TomlSerializationError(#[from] toml::ser::Error),

    /// Convlint was not able to deserialize to
    /// program structure from TOML.
    #[error("Failed to deserialize from TOML: {0}")]
    TomlDeserializationError(#[from] toml::de::Error),

    /// Is thrown if e.g. there is no string before the `:` in
    /// in a commit header.
    #[error("Expected {0}, but was not found")]
    EmptyContent(String),

    /// The parser expected a scope name, but none was found.
    #[error(
        "Missing scope name: a scope name is expected if a `(` is found after the conventional commit type"
    )]
    MissingScopeNameError,

    /// Indicates that the parser expected a character which was not found.
    #[error("Expected `{0}`: {1}")]
    MissingCharacter(char, String),

    /// The parser expeceted a description after the conventional
    /// commit type.
    #[error("Expected a commit description")]
    MissingDescription,

    /// If the parser finds any part in a string, that was not expected.
    #[error("Found a string that was not expected: {0}")]
    UnexpectedContent(String),

    /// Convert an IO Error into a [`ModelError`].
    #[error("IO Error: {source} with path `{path}`")]
    IoError { path: PathBuf, source: io::Error },
}

impl PartialEq for ModelError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MissingDescription, Self::MissingDescription)
            | (Self::MissingScopeNameError, Self::MissingScopeNameError) => true,
            (Self::EmptyContent(s1), Self::EmptyContent(s2)) => s1 == s2,
            (Self::MissingCharacter(c1, s1), Self::MissingCharacter(c2, s2)) => {
                c1 == c2 && s1 == s2
            }
            (Self::TomlDeserializationError(e1), Self::TomlDeserializationError(e2)) => e1 == e2,
            (Self::TomlSerializationError(e1), Self::TomlSerializationError(e2)) => e1 == e2,
            (Self::UnexpectedContent(c1), Self::UnexpectedContent(c2)) => c1 == c2,
            (Self::IoError { source: s1, .. }, Self::IoError { source: s2, .. }) => {
                s1.kind() == s2.kind()
            }
            _ => false,
        }
    }
}

impl Eq for ModelError {}

/// A convenience type for clearer error handling
/// and less verbose return types.
pub type ModelResult<T> = Result<T, ModelError>;
