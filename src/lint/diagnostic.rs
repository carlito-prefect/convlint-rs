use crate::lint::severity::Severity;

/// A representation of a rule violation
/// during during linting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The name of the rule that
    /// was violated.
    pub rule: &'static str,

    /// The severity of the violation.
    pub severity: Severity,

    /// A custom message that can be
    /// passed as more detailed information
    /// on the rule violation.
    pub message: String,
}
