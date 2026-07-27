use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use tracing::{instrument, trace};

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
    /// Fetch commits from a given source.
    ///
    /// If the commit is read from a file, the file content is interpreted as the commit
    /// message. If the commit is read from a command line argument or stdin, the string is
    /// just returned as is. If the commits are read from a git range, a gix repo instance
    /// is created and commits are fetched from history.
    #[allow(clippy::result_large_err)]
    #[instrument(skip(self, git_root))]
    pub fn fetch_commits(&self, git_root: PathBuf) -> SourceResult<Vec<String>> {
        match self {
            Self::File(path) => {
                trace!(file = ?path, "Read commit from file");
                let file_content =
                    fs::read_to_string(path).map_err(|err| SourceError::FileReadError {
                        path: path.to_owned(),
                        source: err,
                    })?;
                trace!("Read from file successfully");
                Ok(vec![file_content])
            }
            Self::Message(msg) => Ok(vec![msg.to_owned()]),
            Self::Stdin => {
                trace!("Start reading from stdin");
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .map_err(SourceError::StdinError)?;
                trace!("Read from stdin successfully");
                Ok(vec![buffer])
            }
            Self::GitRange { from, to } => {
                let repo = GitRepository::new(git_root)?;
                Ok(repo.fetch_git_range_commits(from, to)?)
            }
        }
    }
}
