use serde::{Deserialize, Serialize};
use tracing::{instrument, trace};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how long or short the entire
/// header must be.
///
/// This is an extension to the description
/// length requirement. This also takes into
/// consideration how long scope and the
/// conventional type are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct HeaderLengthConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,

    /// The min length of the header.
    #[serde(rename = "min")]
    minimum: usize,

    /// The max length of the header.
    #[serde(rename = "max")]
    maximum: usize,
}

impl Default for HeaderLengthConfig {
    fn default() -> Self {
        Self {
            level: Severity::Warning,
            minimum: 20,
            maximum: 100,
        }
    }
}

/// A rule that defines how long or short
/// the header may be.
pub struct HeaderLength;

impl Rule for HeaderLength {
    fn id() -> &'static str {
        "header-length"
    }

    #[instrument(skip(commit, config))]
    fn check(commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        trace!("Check if the length of the whole header is valid");
        let header_length_config = config.rules.header_length.clone().unwrap_or_default();
        let header_string = commit.header.to_string();
        if header_string.len() > header_length_config.maximum
            || header_string.len() < header_length_config.minimum
        {
            trace!(
                length = header_string.len(),
                min = header_length_config.minimum,
                max = header_length_config.maximum,
                "The header has an invalid length"
            );
            Some(Diagnostic {
                rule: Self::id(),
                severity: Severity::Warning,
                message: format!(
                    "header is too short or too long: expected min({min}) and max({max}) characters",
                    min = header_length_config.minimum,
                    max = header_length_config.maximum
                ),
                commit: commit.to_string(),
            })
        } else {
            trace!(header = %commit.header, "The header has a valid length");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, header_length::HeaderLength},
    };

    #[fixture]
    fn default_config() -> ConvlintTOML {
        ConvlintTOML::default()
    }

    #[rstest]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "foo".repeat(20),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        None
    )]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "foo".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(Diagnostic {
            rule: "header-length",
            severity: Severity::Warning,
            message: "header is too short or too long: expected min(20) and max(100) characters".into(),
            commit: "fix: foo".into()
        })
    )]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "foo ".repeat(30),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(Diagnostic {
            rule: "header-length",
            severity: Severity::Warning,
            message: "header is too short or too long: expected min(20) and max(100) characters".into(),
            commit: "fix: ".to_string() + &"foo ".repeat(30)
        })
    )]
    fn default_config_check_header_length(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = HeaderLength::check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
