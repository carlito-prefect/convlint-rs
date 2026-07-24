use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::diagnostic::Diagnostic,
    rules::{Rule, rules},
};

/// The [`Linter`] is responsible for applying
/// a given set of rules to a commit message
/// based on a configuration.
pub struct Linter {
    /// The defined rules, which are applied
    /// by the linter.
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    /// Creates a new [`Linter`].
    pub fn new() -> Self {
        Self { rules: rules() }
    }

    /// Apply all rules of the linter to the commit message based on the config.
    ///
    /// All errors from all rules are collected into one [`Vec`] and returned.
    // TODO: add the commit message to the diagnostic to show the user where the error
    // happened
    pub fn lint(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for rule in &self.rules {
            diagnostics.append(&mut rule.check(commit, config));
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::{
        commit::model::{CommitBody, CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, engine::Linter, severity::Severity},
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
                description: "foo ".repeat(20),
                breaking: false
            },
            body: Some(CommitBody { content: "some body content".into() }),
            footers: vec![]
        },
        vec![]
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "false type".into(),
                scope: None,
                description: "foo ".repeat(20),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        vec![
            Diagnostic {
                rule: "body-required",
                severity: Severity::Warning,
                message: String::from("expected the commit message to have a body"),
            },
            Diagnostic {
                rule: "type-exists",
                severity: Severity::Error,
                message: String::from("no valid conventional type: false type")
            },
        ]
    )]
    #[case(
        CommitMessage {
            header: CommitHeader {
                commit_type: "false type".into(),
                scope: None,
                description: "foo ".repeat(30),
                breaking: false
            },
            body: None,
            footers: vec![]
        },
        vec![
            Diagnostic {
                rule: "body-required",
                severity: Severity::Warning,
                message: String::from("expected the commit message to have a body"),
            },
            Diagnostic {
                rule: "description-length",
                severity: Severity::Warning,
                message: String::from("description is too long: expected min(20) and max(100) characters, found chars(120)"),
            },
            Diagnostic {
                rule: "type-exists",
                severity: Severity::Error,
                message: String::from("no valid conventional type: false type")
            },
        ]
    )]
    fn default_config_lint(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Vec<Diagnostic>,
    ) {
        let linter = Linter::new();
        let diagnostics = linter.lint(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
