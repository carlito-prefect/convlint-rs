use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines if a scope must be
/// given or not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRequiredConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,
}

impl Default for ScopeRequiredConfig {
    fn default() -> Self {
        Self {
            level: Severity::Ignore,
        }
    }
}

/// A rule that defines if a
/// scope must be given.
pub struct ScopeRequired;

impl Rule for ScopeRequired {
    fn id(&self) -> &'static str {
        "scope-required"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let scope_required_config = config.rules.scope_required.clone().unwrap_or_default();
        if commit.header.scope.is_some() {
            None
        } else {
            Some(Diagnostic {
                rule: "scope-required",
                severity: scope_required_config.level,
                message: "a scope is required but was not found".into(),
                commit: commit.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitHeader, CommitMessage, CommitScope},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, scope_required::ScopeRequired},
    };

    #[fixture]
    fn default_config() -> ConvlintTOML {
        ConvlintTOML::default()
    }

    #[rstest]
    #[case(CommitMessage {
            header: CommitHeader {
                commit_type: "fix".into(),
                scope: Some(CommitScope {
                    scope_name: "some scope".into()
                }),
                description: "some description".into(),
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
                description: "some description".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(Diagnostic {
            rule: "scope-required",
            severity: Severity::Ignore,
            message: "a scope is required but was not found".into(),
            commit: "fix: some description".into()
        })
    )]
    fn default_config_check_scope_required(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = ScopeRequired.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
