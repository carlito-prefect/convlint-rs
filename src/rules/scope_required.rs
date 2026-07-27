use serde::{Deserialize, Serialize};
use tracing::{instrument, trace};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines if a scope must be
/// given or not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
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
    fn id() -> &'static str {
        "scope-required"
    }

    #[instrument(skip(commit, config))]
    fn check(commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        trace!("Check if a scope is required and provided");
        let scope_required_config = config.rules.scope_required.clone().unwrap_or_default();
        if commit.header.scope.is_some() {
            trace!("Commit has a scope, rule generates no diagnostics");
            None
        } else {
            trace!(header = %commit.header, "Commit does not contain a scope");
            Some(Diagnostic {
                rule: Self::id(),
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
        let diagnostics = ScopeRequired::check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
