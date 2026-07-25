use std::{io, path::PathBuf};

use thiserror::Error;

use crate::error::git_error::GitError;

/// Errors that can occur when reading commit messages from various sources.
#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Failed to read commit from file `{path}`: {source}")]
    FileReadError { path: PathBuf, source: io::Error },
    #[error("Failed to read from stdin: {0}")]
    StdinError(io::Error),
    #[error("Git repository interaction was not successfull: {0}")]
    GitError(#[from] GitError),
}

pub type SourceResult<T> = Result<T, SourceError>;
