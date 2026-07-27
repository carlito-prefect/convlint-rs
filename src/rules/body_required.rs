use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how to treat existent/non-existent
/// commit message body
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct BodyRequiredConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,
}

impl Default for BodyRequiredConfig {
    fn default() -> Self {
        Self {
            level: Severity::Warning,
        }
    }
}

/// A rule that defines if the commit
/// message requires a body.
pub struct BodyRequired;

impl Rule for BodyRequired {
    fn id(&self) -> &'static str {
        "body-required"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let body_required_conf = config.rules.body_required.clone().unwrap_or_default();
        if commit.has_body() {
            None
        } else {
            Some(Diagnostic {
                rule: self.id(),
                severity: body_required_conf.level,
                message: String::from("expected the commit message to have a body"),
                commit: commit.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitBody, CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, body_required::BodyRequired},
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
                description: "foo ".repeat(25),
                breaking: false
            },
            body: Some(CommitBody { content: "some body content".into() }),
            footers: vec![]
        },
        None
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                description: "foo ".repeat(25),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(
            Diagnostic {
                rule: "body-required",
                severity: Severity::Warning,
                message: String::from("expected the commit message to have a body"),
                commit: "feat: ".to_string() + &"foo ".repeat(25)
            }
        )
    )]
    fn default_config_check_body_required(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let check_res = BodyRequired.check(&commit, &default_config);
        assert_eq!(check_res, expected);
    }
}
