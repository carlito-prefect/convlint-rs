use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The severity of a rule violation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    /// Ignores a rule violation.
    #[default]
    Ignore,

    /// Warn if the rule was violated.
    Warning,

    /// Reject the commit message
    /// if the rule was violated.
    Error,
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ignore => write!(f, "{sev}", sev = "IGNORE".blue()),
            Self::Warning => write!(f, "{sev}", sev = "WARNING".yellow()),
            Self::Error => write!(f, "{sev}", sev = "ERROR".red()),
        }
    }
}
