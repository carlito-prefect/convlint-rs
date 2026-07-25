use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use crate::{
    error::source_error::{SourceError, SourceResult},
    git::GitRepository,
};

/// Defines where `convlint` retrieves raw commit messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitSource {
    /// Receive a file's contents, e.g. `./.git/COMMIT_EDITMSG`.
    File(PathBuf),

    /// Receive the commit message from stdin.
    Stdin,

    /// Receive a single commit message string directly.
    Message(String),

    /// Fetch commit messages from a revision range (e.g., from `"HEAD~3"` to `"HEAD"`).
    GitRange { from: String, to: String },
}

impl CommitSource {
    #[allow(clippy::result_large_err)]
    pub fn fetch_commits(&self, git_root: PathBuf) -> SourceResult<Vec<String>> {
        match self {
            Self::File(path) => {
                let file_content =
                    fs::read_to_string(path).map_err(|err| SourceError::FileReadError {
                        path: path.to_owned(),
                        source: err,
                    })?;
                Ok(vec![file_content])
            }
            Self::Message(msg) => Ok(vec![msg.to_owned()]),
            Self::Stdin => {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .map_err(SourceError::StdinError)?;
                Ok(vec![buffer])
            }
            Self::GitRange { from, to } => {
                let repo = GitRepository::new(git_root)?;
                Ok(repo.fetch_git_range_commits(from, to)?)
            }
        }
    }
}
