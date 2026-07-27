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
    match cli.run() {
        Ok(()) => {}
        Err(e) => {
            error!(err = %e, "`convlint` execution did not finish successfully");
            println!("[{sev}] {e}.", sev = Severity::Error);
        }
    }
}
