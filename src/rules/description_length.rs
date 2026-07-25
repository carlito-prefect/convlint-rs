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
pub struct DescriptionLengthConfig {
    /// Defines how to treat a violation.
    pub level: Severity,

    /// The minimum required length of the
    /// description.
    pub minimum: usize,

    /// The maximum lenght of the description.
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
    fn id(&self) -> &'static str {
        "description-length"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Vec<Diagnostic> {
        if commit.header.description.len() > config.rules.description_length.maximum
            || commit.header.description.len() < config.rules.description_length.minimum
        {
            vec![Diagnostic {
                rule: self.id(),
                severity: config.rules.description_length.level,
                message: format!(
                    "description is too {bound}: expected min({min}) and max({max}) characters, found chars({ch})",
                    bound = if commit.header.description.len()
                        > config.rules.description_length.maximum
                    {
                        "long"
                    } else {
                        "short"
                    },
                    min = config.rules.description_length.minimum,
                    max = config.rules.description_length.maximum,
                    ch = commit.header.description.len()
                ),
                commit: format!("{commit}"),
            }]
        } else {
            vec![]
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
        vec![]
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
        vec![
            Diagnostic {
                rule: "description-length",
                severity: Severity::Warning,
                message: String::from("description is too short: expected min(20) and max(100) characters, found chars(3)"),
                commit: "feat: foo".into(),
            }
        ]
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
        vec![
            Diagnostic {
                rule: "description-length",
                severity: Severity::Warning,
                message: String::from("description is too long: expected min(20) and max(100) characters, found chars(120)"),
                commit: "feat: ".to_string() + &"foo ".repeat(30),
            }
        ]
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
        vec![]
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
        vec![]
    )]
    fn default_config_check_description_length(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Vec<Diagnostic>,
    ) {
        let diagnostics = DescriptionLength.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
