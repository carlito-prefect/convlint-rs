use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use tracing::{debug, error, instrument, trace, warn};

use crate::{
    commit::{parser::CommitParser, source::CommitSource},
    config::{CONF_FILE_NAME, ConvlintTOML},
    error::config_error::ConfigError,
    lint::{engine::Linter, severity::Severity},
};

/// Yet another conventional commit linter.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None, author, )]
pub struct ConvlintCli {
    /// Verbosity level (-v, -vv, -vvv, -q)
    #[command(flatten)]
    pub verbose: Verbosity<WarnLevel>,

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
    #[instrument(skip(self), fields(cmd = ?self.sub_command))]
    pub fn run(self) -> Result<i32, Box<dyn Error>> {
        let mut exit_code = 0;
        match self.sub_command {
            ConvlintSubcommand::Init => {
                debug!(config_file = %CONF_FILE_NAME, "Write the default config to the config file");
                let config = ConvlintTOML::default();
                if fs::exists(Path::new(CONF_FILE_NAME))? {
                    error!(
                        "Configuration file already exists. Aborted writing default configuration to file"
                    );
                    return Err(Box::new(ConfigError::ConfigurationWriteError(
                        String::from("the `convlint init` expects no existing configuraton file"),
                    )));
                }
                config.write_to_file(Path::new(CONF_FILE_NAME))?;
                debug!("Finished writing to configuration file");
                Ok(exit_code)
            }
            ConvlintSubcommand::Lint(lint_args) => {
                debug!("Parse commit(s) and apply rules");
                trace!("Read config from file");
                let config = ConvlintTOML::from_file(&lint_args.config)?;
                let source = if lint_args.edit.is_some() {
                    trace!(file = ?lint_args.edit, "Select a file as commit source");
                    #[allow(clippy::or_fun_call)]
                    CommitSource::File(
                        lint_args
                            .edit
                            .unwrap_or(PathBuf::from("./.git/COMMIT_EDITMSG")),
                    )
                } else if lint_args.from.is_some() {
                    trace!(from = ?lint_args.from, to = ?lint_args.to, "Select a git range as commit source");
                    #[allow(clippy::or_fun_call)]
                    CommitSource::GitRange {
                        from: lint_args.from.unwrap_or("HEAD~".into()),
                        to: lint_args.to.unwrap_or("HEAD".into()),
                    }
                } else if lint_args.msg.is_some() {
                    trace!(
                        msg = &lint_args.msg,
                        "Select a command line string as commit source"
                    );
                    CommitSource::Message(lint_args.msg.unwrap_or_default())
                } else {
                    trace!("Select stdin as commit source");
                    CommitSource::Stdin
                };
                trace!("Fetch all commits");
                let commits_str = source.fetch_commits(lint_args.directory)?;
                let parser = CommitParser::new(commits_str);
                trace!("Parse all commits into a `CommitMessage`");
                let commits = parser.parse_commits()?;

                trace!(commit_count = %commits.len(), "Lint all commits");
                let diagnostics = commits.iter().map(|commit| Linter::lint(commit, &config));

                trace!(diagnostics_count = %diagnostics.len(), "Output all diagnostics");
                let linter_diagnostics = diagnostics.into_iter().flatten().collect::<Vec<_>>();
                for diagnostic in linter_diagnostics {
                    match diagnostic.severity {
                        Severity::Ignore => continue,
                        Severity::Warning => {}
                        Severity::Error => exit_code = 1,
                    }
                    println!("{diagnostic}");
                }
                debug!("Finished linting commits");
                Ok(exit_code)
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
    #[arg(
        short = 'e',
        long,
        num_args = 0..=1,
        default_missing_value = "./.git/COMMIT_EDITMSG"
    )]
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
