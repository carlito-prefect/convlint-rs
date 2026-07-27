use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how to treat the length of the commit's
/// header's description.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct DescriptionLengthConfig {
    /// Defines how to treat a violation.
    pub level: Severity,

    /// The minimum required length of the
    /// description.
    #[serde(rename = "min")]
    pub minimum: usize,

    /// The maximum lenght of the description.
    #[serde(rename = "max")]
    pub maximum: usize,
}

impl Default for DescriptionLengthConfig {
    fn default() -> Self {
        Self {
            level: Severity::Warning,
            minimum: 20,
            maximum: 100,
        }
    }
}

/// A rule that checks the length of the
/// commit description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptionLength;

impl Rule for DescriptionLength {
    fn id() -> &'static str {
        "description-length"
    }

    fn check(commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let description_length_config = config.rules.description_length.unwrap_or_default();
        if commit.header.description.len() > description_length_config.maximum
            || commit.header.description.len() < description_length_config.minimum
        {
            Some(Diagnostic {
                rule: Self::id(),
                severity: description_length_config.level,
                message: format!(
                    "description is too {bound}: expected min({min}) and max({max}) characters, found chars({ch})",
                    bound = if commit.header.description.len() > description_length_config.maximum {
                        "long"
                    } else {
                        "short"
                    },
                    min = description_length_config.minimum,
                    max = description_length_config.maximum,
                    ch = commit.header.description.len()
                ),
                commit: commit.to_string(),
            })
        } else {
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
        rules::{Rule, description_length::DescriptionLength},
    };

    #[fixture]
    fn default_config() -> ConvlintTOML {
        ConvlintTOML::default()
    }

    #[rstest]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                description: "added new features for parser".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        None
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                description: "foo".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(
            Diagnostic {
                rule: "description-length",
                severity: Severity::Warning,
                message: String::from("description is too short: expected min(20) and max(100) characters, found chars(3)"),
                commit: "feat: foo".into(),
            }
        )
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                description: "foo ".repeat(30),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(
            Diagnostic {
                rule: "description-length",
                severity: Severity::Warning,
                message: String::from("description is too long: expected min(20) and max(100) characters, found chars(120)"),
                commit: "feat: ".to_string() + &"foo ".repeat(30),
            }
        )
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                // exactly 20 chars
                description: "foo ".repeat(5),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        None
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                // exaclty 100 chars
                description: "foo ".repeat(25),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        None
    )]
    fn default_config_check_description_length(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = DescriptionLength::check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
