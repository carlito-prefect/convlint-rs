use serde::{Deserialize, Serialize};

use crate::{
    commit::model::CommitMessage,
    config::ConvlintTOML,
    lint::diagnostic::Diagnostic,
    rules::{
        body_required::{BodyRequired, BodyRequiredConfig},
        description_length::{DescriptionLength, DescriptionLengthConfig},
        type_exists::{TypeExists, TypeExistsConfig},
    },
};

pub mod body_required;
pub mod description_length;
pub mod type_exists;

#[must_use]
pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(BodyRequired),
        Box::new(DescriptionLength),
        Box::new(TypeExists),
    ]
}

/// Defines functionality all rules must provide
/// to check for violations.
pub trait Rule {
    /// Returns the id/name of the rule.
    fn id(&self) -> &'static str;

    /// Check if the commit message violates a rule based
    /// on the config.
    // TODO: check if a single diagnostic is also applicable
    // if so remove the vec
    fn check(&self, commit: &CommitMessage, config: &ConvlintTOML) -> Vec<Diagnostic>;
}

/// Holds all rule types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct RulesConfig {
    /// Defines how to treat existent/non-existent
    /// conventional types.
    pub type_exists: TypeExistsConfig,

    /// Defines how to treat a commit message's header's
    /// description length.
    pub description_length: DescriptionLengthConfig,

    /// Defines how to treat an existent/non-existent commit
    /// message body.
    pub body_required: BodyRequiredConfig,
}
