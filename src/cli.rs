use std::{
    error::Error,
    path::{Path, PathBuf},
};

use clap::{Args, Parser, Subcommand};

use crate::{
    commit::{parser::CommitParser, source::CommitSource},
    config::{CONF_FILE_NAME, ConvlintTOML},
    lint::engine::Linter,
};

/// Yet another conventional commit linter.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None, author, )]
pub struct ConvlintCli {
    /// The convlint command to execute
    #[clap(subcommand)]
    sub_command: ConvlintSubcommand,
}

impl ConvlintCli {
    /// Runs the [`ConvlintSubcommand`] parsed from [`ConvlintCli`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the init
    /// config could not been written to the current directory.
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self.sub_command {
            ConvlintSubcommand::Init => {
                let config = ConvlintTOML::default();
                config.write_to_file(Path::new(CONF_FILE_NAME))?;
                Ok(())
            }
            ConvlintSubcommand::Lint(lint_args) => {
                let config = ConvlintTOML::from_file(&lint_args.config)?;
                let source = if lint_args.edit.is_some() {
                    #[allow(clippy::or_fun_call)]
                    CommitSource::File(
                        lint_args
                            .edit
                            .unwrap_or(PathBuf::from("./.git/COMMIT_EDITMSG")),
                    )
                } else if lint_args.from.is_some() {
                    #[allow(clippy::or_fun_call)]
                    CommitSource::GitRange {
                        from: lint_args.from.unwrap_or("HEAD~".into()),
                        to: lint_args.to.unwrap_or("HEAD".into()),
                    }
                } else if lint_args.msg.is_some() {
                    CommitSource::Message(lint_args.msg.unwrap_or_default())
                } else {
                    CommitSource::Stdin
                };
                let commits_str = source.fetch_commits(lint_args.directory)?;
                let parser = CommitParser::new(commits_str);
                let commits = parser.parse_commits()?;
                let linter = Linter::new();

                let diagnostics = commits.iter().map(|commit| linter.lint(commit, &config));

                let linter_diagnostics = diagnostics.into_iter().flatten().collect::<Vec<_>>();
                for diagnostic in linter_diagnostics {
                    println!("{diagnostic}");
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConvlintSubcommand {
    /// Output the default configuration into `Convlint.toml`
    Init,

    /// Lint commits
    Lint(LintArgs),
}

#[derive(Debug, Clone, Args)]
pub struct LintArgs {
    /// Edit the last commit message from the
    /// specified file or fallback to `./.git/COMMIT_EDITMSG`
    #[arg(short = 'e', long)]
    pub edit: Option<PathBuf>,

    /// Lower end of the commit range to lint
    #[arg(short = 'f', long, conflicts_with = "edit")]
    pub from: Option<String>,

    /// Upper end of the commit range to lint
    #[arg(short = 't', long, conflicts_with = "edit", requires = "from")]
    pub to: Option<String>,

    /// Directory to execute in
    #[arg(short = 'd', long, default_value = ".")]
    pub directory: PathBuf,

    /// Specify the configuration to use
    #[arg(short = 'c', long, default_value = CONF_FILE_NAME)]
    pub config: PathBuf,

    /// A message to read from if no file or git range is specified
    pub msg: Option<String>,
}
