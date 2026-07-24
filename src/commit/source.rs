use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum CommitSource {
    File(PathBuf),
    Stdin,
    Message(String),
    GitRange { from: String, to: String },
}
