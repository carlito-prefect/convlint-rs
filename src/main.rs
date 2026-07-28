use std::process::exit;

use clap::Parser;
use convlint::{cli::ConvlintCli, lint::severity::Severity};
use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() {
    let cli = ConvlintCli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(cli.verbose)
        .pretty()
        .init();
    let exit_code = match cli.run() {
        Ok(exit_code) => exit_code,
        Err(e) => {
            error!(err = %e, "`convlint` execution did not finish successfully");
            println!("[{sev}] {e}.", sev = Severity::Error);
            1
        }
    };
    exit(exit_code);
}
