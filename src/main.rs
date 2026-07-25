use clap::Parser;
use convlint::{cli::ConvlintCli, lint::severity::Severity};

#[tokio::main]
async fn main() {
    let cli = ConvlintCli::parse();
    match cli.run().await {
        Ok(()) => {}
        Err(e) => println!("[{sev}] {e}.", sev = Severity::Error),
    }
}
