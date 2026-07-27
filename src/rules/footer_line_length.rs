use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how short or long a footer line may be
/// and how to treat violations of this length range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct FooterLineLengthConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,

    /// The min length of a footer line.
    #[serde(rename = "min")]
    minimum: usize,

    /// The max length of a footer line.
    #[serde(rename = "max")]
    maximum: usize,
}

impl Default for FooterLineLengthConfig {
    fn default() -> Self {
        Self {
            level: Severity::Ignore,
            minimum: 20,
            maximum: 100,
        }
    }
}

pub struct FooterLineLength;

impl Rule for FooterLineLength {
    fn id() -> &'static str {
        "footer-line-length"
    }

    fn check(commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let footer_line_length_config = config.rules.footer_line_length.clone().unwrap_or_default();
        let mut too_long_lines = Vec::new();
        for footer in &commit.footers {
            for line in footer.value.lines() {
                if line.len() > footer_line_length_config.maximum
                    || line.len() < footer_line_length_config.minimum
                {
                    too_long_lines.push(line);
                }
            }
        }
        if too_long_lines.is_empty() {
            None
        } else {
            Some(Diagnostic {
                rule: Self::id(),
                severity: footer_line_length_config.level,
                message: format!(
                    "footer lines are too long or too short: expected min({min}) and max({max}) characters",
                    min = footer_line_length_config.minimum,
                    max = footer_line_length_config.maximum,
                ),
                commit: commit.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitFooter, CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, footer_line_length::FooterLineLength},
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
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![CommitFooter {
                breaking: false,
                token: "footer".into(),
                value: "some\nmore\nlines".into(),
            }]
        },
        Some(Diagnostic {
            rule: "footer-line-length",
            severity: Severity::Ignore,
            message: "footer lines are too long or too short: expected min(20) and max(100) characters".into(),
            commit: "fix: some description\n\nfooter: some\nmore\nlines".into()
        })
    )]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![CommitFooter {
                breaking: false,
                token: "footer".into(),
                value: "foo ".repeat(30) + "\n" + &"foo ".repeat(20),
            }]
        },
        Some(Diagnostic {
            rule: "footer-line-length",
            severity: Severity::Ignore,
            message: "footer lines are too long or too short: expected min(20) and max(100) characters".into(),
            commit: "fix: some description\n\nfooter: ".to_string() + &"foo ".repeat(30) + "\n" + &"foo ".repeat(20)
        })
    )]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![CommitFooter {
                breaking: false,
                token: "footer".into(),
                value: "foo ".repeat(20) + "\n" + &"foo ".repeat(20),
            }]
        },
        None
    )]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        None
    )]
    fn default_config_check_footer_line_lengths(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = FooterLineLength::check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
