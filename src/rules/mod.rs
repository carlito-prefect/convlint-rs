use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::diagnostic::Diagnostic,
    rules::{
        body_line_length::BodyLineLengthConfig, body_required::BodyRequiredConfig,
        breaking_change_consistency::BreakingChangeConsistencyConfig,
        description_length::DescriptionLengthConfig, footer_line_length::FooterLineLengthConfig,
        footer_required::FooterRequiredConfig, header_length::HeaderLengthConfig,
        scope_required::ScopeRequiredConfig, type_exists::TypeExistsConfig,
    },
};

pub mod body_line_length;
pub mod body_required;
pub mod breaking_change_consistency;
pub mod description_length;
pub mod description_required;
pub mod footer_line_length;
pub mod footer_required;
pub mod header_length;
pub mod scope_required;
pub mod type_exists;

/// Defines functionality all rules must provide
/// to check for violations.
pub trait Rule: Send + Sync {
    /// Returns the id/name of the rule.
    fn id() -> &'static str
    where
        Self: Sized;

    /// Check if the commit message violates a rule based
    /// on the config.
    fn check(commit: &CommitMessage, config: &ConvlintTOML) -> Option<Diagnostic>
    where
        Self: Sized;
}

/// Holds all rule types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct RulesConfig {
    /// Defines how to treat existent/non-existent
    /// conventional types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_exists: Option<TypeExistsConfig>,

    /// Defines how to treat a commit message's header's
    /// description length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_length: Option<DescriptionLengthConfig>,

    /// Defines how to treat an existent/non-existent commit
    /// message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_required: Option<BodyRequiredConfig>,

    /// Defines how to treat the length of the individual
    /// body lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_line_length: Option<BodyLineLengthConfig>,

    /// Defines how to treat inconsistencies in breaking
    /// change indications in header and footers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breaking_change_consistency: Option<BreakingChangeConsistencyConfig>,

    /// Defines how to treat the length of the individual
    /// footer lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_line_length: Option<FooterLineLengthConfig>,

    /// Defines how to treat existent/non-existent footes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_required: Option<FooterRequiredConfig>,

    /// Defines how to treat the length of the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_length: Option<HeaderLengthConfig>,

    /// Defines how to treat existent/non-existent scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_required: Option<ScopeRequiredConfig>,
}
