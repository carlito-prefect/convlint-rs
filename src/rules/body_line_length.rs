use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::{diagnostic::Diagnostic, severity::Severity},
    rules::Rule,
};

/// Defines how long or short each line
/// in the body must be.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyLineLengthConfig {
    /// Defines how to treat violations of this rule.
    level: Severity,

    /// The min length of a body line.
    minimum: usize,

    /// The max length of a body line.
    maximum: usize,
}

impl Default for BodyLineLengthConfig {
    fn default() -> Self {
        Self {
            level: Severity::Warning,
            minimum: 20,
            maximum: 100,
        }
    }
}

/// A rule that defines how long or short
/// each line of the body may be.
pub struct BodyLineLength;

impl Rule for BodyLineLength {
    fn id(&self) -> &'static str {
        "body-line-length"
    }

    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic> {
        let body_line_length_config = config.rules.body_line_length.clone().unwrap_or_default();
        let Some(body) = &commit.body else {
            return None;
        };
        let mut too_long_lines = Vec::new();
        for line in body.content.lines() {
            if line.len() > body_line_length_config.maximum
                || line.len() < body_line_length_config.minimum
            {
                too_long_lines.push(line);
            }
        }
        if too_long_lines.is_empty() {
            None
        } else {
            Some(Diagnostic {
                rule: self.id(),
                severity: body_line_length_config.level,
                message: format!(
                    "body lines are too long or too short: expected min({min}) and max({max}) characters",
                    min = body_line_length_config.minimum,
                    max = body_line_length_config.maximum,
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
        commit::model::{CommitBody, CommitHeader, CommitMessage},
        config::ConvlintTOML,
        lint::{diagnostic::Diagnostic, severity::Severity},
        rules::{Rule, body_line_length::BodyLineLength},
    };

    #[fixture]
    fn default_config() -> ConvlintTOML {
        ConvlintTOML::default()
    }

    #[rstest]
    #[case(CommitMessage {
            header: CommitHeader { commit_type: "fix".into(), scope: None, description: "some".into(), breaking: false },
            body: Some(CommitBody { content: "some content".into() }),
            footers: vec![],
        },
        Some(Diagnostic {
            severity: Severity::Warning,
            commit: "fix: some\n\nsome content".into(),
            rule: "body-line-length",
            message: "body lines are too long or too short: expected min(20) and max(100) characters".into()
        })
    )]
    #[case(CommitMessage {
            header: CommitHeader { commit_type: "fix".into(), scope: None, description: "some".into(), breaking: false },
            body: Some(CommitBody { content: "foo ".repeat(30) }),
            footers: vec![],
        },
        Some(Diagnostic {
            severity: Severity::Warning,
            commit: "fix: some\n\n".to_string() + &"foo ".repeat(30),
            rule: "body-line-length",
            message: "body lines are too long or too short: expected min(20) and max(100) characters".into()
        })
    )]
    #[case(CommitMessage {
            header: CommitHeader { commit_type: "fix".into(), scope: None, description: "some".into(), breaking: false },
            body: Some(CommitBody { content: "foo ".repeat(25) }),
            footers: vec![],
        },
        None
    )]
    #[case(CommitMessage {
            header: CommitHeader { commit_type: "fix".into(), scope: None, description: "some".into(), breaking: false },
            body: None,
            footers: vec![],
        },
        None
    )]
    fn default_config_check_body_line_lengths(
        #[case] commit: CommitMessage,
        default_config: ConvlintTOML,
        #[case] expected: Option<Diagnostic>,
    ) {
        let diagnostics = BodyLineLength.check(&commit, &default_config);
        assert_eq!(diagnostics, expected);
    }
}
