use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct ConvlintCli {
    #[clap(subcommand)]
    suc_command: ConvlintSubcommand,
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
    #[arg(short = 'e', long, default_value = "./.git/COMMIT_EDITMSG")]
    pub(crate) edit: PathBuf,

    /// Lower end of the commit range to lint
    #[arg(short = 'f', long, conflicts_with = "edit")]
    pub(crate) from: Option<String>,

    /// Upper end of the commit range to lint
    #[arg(short = 't', long, conflicts_with = "edit")]
    pub(crate) to: Option<String>,

    /// Directory to execute in
    #[arg(short = 'd', long, default_value = ".")]
    pub(crate) directory: PathBuf,

    /// Specify the configuration to use
    #[arg(short = 'c', long, default_value = ".")]
    pub(crate) config: PathBuf,
}
