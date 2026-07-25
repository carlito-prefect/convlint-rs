use std::fmt::{Display, Write};

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

    /// The commit the diagnostic was
    /// found in.
    pub commit: String,
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut commit_repr = String::new();
        for line in self.commit.lines() {
            let _ = writeln!(commit_repr, "   {line}");
        }
        write!(
            f,
            "[{sev}] {msg} from rule `{r}`\n\n{commit_repr}\n",
            sev = self.severity,
            msg = self.message,
            r = self.rule
        )
    }
}
