use std::io;

use thiserror::Error;

/// Errors that occur during config management.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Occurs if a rule has a minimum and a maximum value
    /// e.g. description length and the minimum value is
    /// greater than the maximum value.
    #[error(
        "Maximum {rule} length must be greater equals the minimum length, but found: min({min}), max({max})"
    )]
    InvalidLengthRange {
        rule: String,
        min: usize,
        max: usize,
    },

    /// Occurs if a rule has a minimum and a maximum value
    /// e.g. description length and the maximum value is 0.
    #[error("Maximum {_0} length must be at least 1")]
    MaxZeroLength(String /* rule name */),

    /// Convert an IO Error to a [`ConfigError`]
    #[error("Io Error: {_0}")]
    IoError(#[from] io::Error),

    /// Convlint was not able to serialize
    /// program structure to TOML.
    #[error("Failed to serialize to TOML: {_0}")]
    TomlSerializationError(#[from] toml::ser::Error),

    /// Convlint was not able to deserialize to
    /// program structure from TOML.
    #[error("Failed to deserialize from TOML: {_0}")]
    TomlDeserializationError(#[from] toml::de::Error),
}

impl PartialEq for ConfigError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IoError(e1), Self::IoError(e2)) => e1.kind() == e2.kind(),
            (
                Self::InvalidLengthRange {
                    rule: r1,
                    min: min1,
                    max: max1,
                },
                Self::InvalidLengthRange {
                    rule: r2,
                    min: min2,
                    max: max2,
                },
            ) => r1 == r2 && min1 == min2 && max1 == max2,
            (Self::MaxZeroLength(r1), Self::MaxZeroLength(r2)) => r1 == r2,
            (Self::TomlDeserializationError(e1), Self::TomlDeserializationError(e2)) => e1 == e2,
            (Self::TomlSerializationError(e1), Self::TomlSerializationError(e2)) => e1 == e2,
            _ => false,
        }
    }
}

pub type ConfigResult<T> = Result<T, ConfigError>;
