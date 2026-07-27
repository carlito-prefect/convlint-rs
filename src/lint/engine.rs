use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::diagnostic::Diagnostic,
    rules::{
        Rule, body_line_length::BodyLineLength, body_required::BodyRequired,
        breaking_change_consistency::BreakingChangeConsistency,
        description_length::DescriptionLength, footer_line_length::FooterLineLength,
        footer_required::FooterRequired, header_length::HeaderLength,
        scope_required::ScopeRequired, type_exists::TypeExists,
    },
};

/// The [`Linter`] is responsible for applying
/// a given set of rules to a commit message
/// based on a configuration.
pub struct Linter;

impl Linter {
    /// Apply all rules of the linter to the commit message based on the config.
    ///
    /// All errors from all rules are collected into one [`Vec`] and returned.
    #[must_use]
    pub fn lint(commit: &CommitMessage, config: &ConvlintTOML) -> Vec<Diagnostic> {
        vec![
            BodyLineLength::check(commit, config),
            BodyRequired::check(commit, config),
            BreakingChangeConsistency::check(commit, config),
            DescriptionLength::check(commit, config),
            FooterLineLength::check(commit, config),
            FooterRequired::check(commit, config),
            HeaderLength::check(commit, config),
            ScopeRequired::check(commit, config),
            TypeExists::check(commit, config),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
