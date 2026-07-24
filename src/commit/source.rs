use std::path::PathBuf;

/// Defines the source `convlint` gets
/// its raw commit messages from.
#[derive(Debug, Clone)]
pub enum CommitSource {
    /// Receive a file's contents e.g.
    /// `./.git/COMMIT_EDITMSG`.
    File(PathBuf),

    /// Receive the commit message from stdin.
    Stdin,

    /// Receive a single commit message.
    Message(String),

    /// Get a git log range and collect the
    /// commit messages from this.
    GitRange { from: String, to: String },
}
