use std::path::PathBuf;

use gix::{Id, Repository};

use crate::error::git_error::{GitError, GitResult};

/// A wrapper around the [`gix::Repository`] for
/// a nicer API.
#[derive(Debug, Clone)]
pub struct GitRepository {
    /// The gix repository.
    pub gix_repo: Repository,
}

impl GitRepository {
    /// Creates a new [`GitRepository`].
    ///
    /// # Errors
    ///
    /// This function will return an error if there is no
    /// git repository found at the specified path.
    #[allow(clippy::result_large_err)]
    pub fn new(root: PathBuf) -> GitResult<Self> {
        Ok(Self {
            gix_repo: gix::discover(root)?,
        })
    }

    /// Fetches all commits in a given range and returns their messages.
    ///
    /// If there is no `to` specified at function call, `HEAD` is the
    /// default `to` specification passed from the CLI.
    ///
    /// # Errors
    ///
    /// This function will return an error if convlint could not iterate
    /// over the range, an object is not a commit or the commit is invalid.
    #[allow(clippy::result_large_err)]
    pub fn fetch_git_range_commits(&self, from: &str, to: &str) -> GitResult<Vec<String>> {
        let head = self.gix_repo.rev_parse_single(to)?;
        let base = self.gix_repo.rev_parse_single(from)?;

        self.base_before_head(base, head)?;

        let walk = self.gix_repo.rev_walk([head]).with_hidden([base]).all()?;
        let mut commits = Vec::new();

        for info in walk {
            let info = info?;
            let commit = self.gix_repo.find_commit(info.id)?;
            let header = commit.message()?.title;
            let body = commit
                .message()?
                .body()
                .map(|b| b.to_string())
                .unwrap_or_default();
            if body.is_empty() {
                commits.push(header.to_string());
            } else {
                commits.push(header.to_string() + "\n\n" + &body);
            }
        }

        Ok(commits)
    }

    /// Checks if the commit reference passed to `from` is before `to`.
    #[allow(clippy::result_large_err)]
    fn base_before_head<'a>(&self, base: Id<'a>, head: Id<'a>) -> GitResult<bool> {
        let walk = self.gix_repo.rev_walk([head]).all()?;

        let mut found = false;

        for info in walk {
            let info = info?;

            if info.id == base {
                found = true;
                break;
            }
        }

        if !found {
            return Err(GitError::HeadBaseCommitError(
                base.to_string(),
                head.to_string(),
            ));
        }
        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use tokio::fs;

    use rstest::{fixture, rstest};
    use tempfile::TempDir;

    use crate::{error::git_error::GitError, git::GitRepository};

    #[fixture]
    async fn initialized_temp_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let _ = gix::init(temp_dir.path()).unwrap();
        let msg = "some message content\n\nsome body\n";
        fs::write(temp_dir.path().join("file.txt"), msg)
            .await
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "."])
            .status()
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.com",
                "commit",
                "-m",
                msg,
            ])
            .status()
            .unwrap();

        fs::write(temp_dir.path().join("file1.txt"), msg)
            .await
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "."])
            .status()
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.com",
                "commit",
                "-m",
                msg,
            ])
            .status()
            .unwrap();
        temp_dir
    }

    #[fixture]
    fn uninitialized_temp_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[rstest]
    #[tokio::test]
    #[case("HEAD~", "HEAD", vec!["some message content\n\nsome body\n".into()])]
    #[case("HEAD", "HEAD", vec![])]
    async fn fetch_git_range_commits_successfull(
        #[case] from: &str,
        #[case] to: &str,
        #[case] expected: Vec<String>,
        initialized_temp_dir: impl Future<Output = TempDir>,
    ) {
        let initialized_temp_dir = initialized_temp_dir.await;
        let git_repo = GitRepository::new(initialized_temp_dir.path().to_path_buf());
        assert!(git_repo.is_ok());
        let repo = git_repo.unwrap();
        let commits = repo.fetch_git_range_commits(from, to);
        assert!(commits.is_ok());
        assert_eq!(commits.unwrap(), expected);
    }

    #[rstest]
    #[tokio::test]
    #[case("HEAD", "HEAD~")]
    async fn fetch_git_range_commits_head_before_base(
        #[case] from: &str,
        #[case] to: &str,
        initialized_temp_dir: impl Future<Output = TempDir>,
    ) {
        let initialized_temp_dir = initialized_temp_dir.await;
        let git_repo = GitRepository::new(initialized_temp_dir.path().to_path_buf());
        assert!(git_repo.is_ok());
        let repo = git_repo.unwrap();
        let commits_res = repo.fetch_git_range_commits(from, to);
        assert!(commits_res.is_err());
        assert!(matches!(
            commits_res.unwrap_err(),
            GitError::HeadBaseCommitError(_, _)
        ));
    }

    #[rstest]
    #[tokio::test]
    async fn git_repo_new_success(initialized_temp_dir: impl Future<Output = TempDir>) {
        let initialized_temp_dir = initialized_temp_dir.await;
        let git_repo = GitRepository::new(initialized_temp_dir.path().to_path_buf());
        assert!(git_repo.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn git_repo_new_fail(uninitialized_temp_dir: TempDir) {
        let git_repo = GitRepository::new(uninitialized_temp_dir.path().to_path_buf());
        assert!(git_repo.is_err());
    }
}
