use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines if a breaking indication should be in a footer, if any,
/// if there is `!` in the header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct BreakingChangeConsistencyConfig {
    /// Defines how to treat consitency violations
    level: Severity,
}

impl Default for BreakingChangeConsistencyConfig {
    fn default() -> Self {
        Self {
            level: Severity::Ignore,
        }
    }
}

/// Check if there is a breaking indication in a footer,
/// if any, if the header is marked as breaking.
pub struct BreakingChangeConsistency;

impl Rule for BreakingChangeConsistency {
    fn id(&self) -> &'static str {
        "breaking-change-consistency"
    }
    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let breaking_change_consistency_config = config
            .rules
            .breaking_change_consistency
            .clone()
            .unwrap_or_default();
        if !commit.has_footers()
            || commit.header.breaking == commit.footers.iter().any(|f| f.breaking)
        {
            return None;
        }
        Some(Diagnostic {
            rule: self.id(),
            severity: breaking_change_consistency_config.level,
            message: "breaking indications in header and footers are not consistent".into(),
            commit: commit.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitFooter, CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, breaking_change_consistency::BreakingChangeConsistency},
    };

    #[fixture]
    fn default_config() -> ConvlintTOML {
        ConvlintTOML::default()
    }

    #[rstest]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some".into(),
                breaking: true
            },
            body: None,
            footers: vec![
                CommitFooter {
                    token: "breaking".into(),
                    value: "some more".into(),
                    breaking: true
                }
            ]
        },
        None
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some".into(),
                breaking: false
            },
            body: None,
            footers: vec![
                CommitFooter {
                    token: "breaking".into(),
                    value: "some more".into(),
                    breaking: true
                }
            ]
        },
        Some(
            Diagnostic {
                rule: "breaking-change-consistency",
                severity: Severity::Ignore,
                message: "breaking indications in header and footers are not consistent".into(),
                commit: "fix: some\n\nbreaking: some more".into()
            }
        )
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some".into(),
                breaking: true
            },
            body: None,
            footers: vec![
                CommitFooter {
                    token: "some".into(),
                    value: "some more".into(),
                    breaking: false
                }
            ]
        },
        Some(
            Diagnostic {
                rule: "breaking-change-consistency",
                severity: Severity::Ignore,
                message: "breaking indications in header and footers are not consistent".into(),
                commit: "fix!: some\n\nsome: some more".into()
            }
        )
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: None,
                description: "some".into(),
                breaking: false
            },
            body: None,
            footers: vec![
                CommitFooter {
                    token: "some".into(),
                    value: "some more".into(),
                    breaking: false
                }
            ]
        },
        None
    )]
    fn default_config_check_breaking_change_consistency(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = BreakingChangeConsistency.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
