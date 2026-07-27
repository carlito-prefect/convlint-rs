use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how to treat existent/non-existent
/// commit message footers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct FooterRequiredConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,
}

impl Default for FooterRequiredConfig {
    fn default() -> Self {
        Self {
            level: Severity::Warning,
        }
    }
}

pub struct FooterRequired;

impl Rule for FooterRequired {
    fn id(&self) -> &'static str {
        "footer-required"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let footer_required_conf = config.rules.footer_required.clone().unwrap_or_default();
        if commit.has_footers() {
            None
        } else {
            Some(Diagnostic {
                rule: self.id(),
                severity: footer_required_conf.level,
                message: "expected the commit message to have at least one footer".into(),
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
        rules::{Rule, footer_required::FooterRequired},
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
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![
                CommitFooter {
                    breaking: false,
                    token: "footer".into(),
                    value: "some footer content".into()
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
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(Diagnostic {
            rule: "footer-required",
            severity: Severity::Warning,
            message: "expected the commit message to have at least one footer".into(),
            commit: "fix: some description".into()
        })
    )]

    fn default_config_check_footer_required(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = FooterRequired.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
