use gix::{
    ObjectId, discover,
    object::find::existing::with_conversion,
    revision::{spec::parse, walk},
};
use gix_object::decode;
use thiserror::Error;

/// Errors that occur during git operations using `gix`.
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Failed to find git repository in `{0}`")]
    Discover(#[from] discover::Error),

    #[error("Failed to find commit: {0}")]
    RevParseSingle(#[from] parse::single::Error),

    #[error("Failed to create a walker over git history: {0}")]
    GitLogWalk(#[from] walk::Error),

    #[error("Failed to iterate over found commits: {0}")]
    IterWalk(#[from] walk::iter::Error),

    #[error("Failed to find commit: {0}")]
    CommitNotFound(#[from] with_conversion::Error),

    #[error("No commit message was found on commit: {0}")]
    CommitHasNoMessage(#[from] decode::Error),

    #[error("Commit with id `{0}` has no gix-commit-message body.")]
    CommitMessageHasNoBody(ObjectId),

    #[error("`from` commit `{0}` is not before `to` commit `{1}`")]
    HeadBaseCommitError(String, String),
}

pub type GitResult<T> = Result<T, GitError>;
