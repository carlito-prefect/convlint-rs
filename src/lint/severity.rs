use serde::{Deserialize, Serialize};

/// The severity of a rule violation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    /// Ignores a rule violation.
    Ignore,

    /// Warn if the rule was violated.
    Warning,

    /// Reject the commit message
    /// if the rule was violated.
    Error,
}
