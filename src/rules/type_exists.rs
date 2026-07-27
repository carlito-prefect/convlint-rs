use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how to treat existent/non-existent conventional types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct TypeExistsConfig {
    /// Defines the severity of type existence
    /// violations.
    pub level: Severity,

    /// Defines all allowed types.
    ///
    /// Default types are equivalent to
    /// the conventional types defined by
    /// <https://www.conventionalcommits.org/en/v1.0.0/>.
    pub allowed: Vec<String>,
}

impl Default for TypeExistsConfig {
    fn default() -> Self {
        Self {
            level: Severity::Error,
            allowed: vec!["feat".into(), "fix".into()],
        }
    }
}

/// A rule that defines if a type must exist.
pub struct TypeExists;

impl Rule for TypeExists {
    fn id(&self) -> &'static str {
        "type-exists"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let type_exists_config = config.rules.type_exists.clone().unwrap_or_default();
        for allowed_type in &type_exists_config.allowed {
            if &commit.header.commit_type == allowed_type {
                return None;
            }
        }
        Some(Diagnostic {
            rule: self.id(),
            severity: type_exists_config.level,
            message: format!("no valid conventional type: {}", commit.header.commit_type),
            commit: commit.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, type_exists::TypeExists},
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
                description: "added new features".into(),
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
                commit_type: "fix".into(),
                scope: None,
                description: "added new features".into(),
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
                commit_type: "false type".into(),
                scope: None,
                description: "added new features".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        Some(
            Diagnostic {
                rule: "type-exists",
                severity: Severity::Error,
                message: String::from("no valid conventional type: false type"),
                commit: "false type: added new features".into(),
            }
        )
    )]
    fn default_config_check_type_exists(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = TypeExists.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
