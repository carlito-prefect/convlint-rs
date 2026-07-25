use clap::Parser;
use convlint::{cli::ConvlintCli, lint::severity::Severity};

// TODO make async
fn main() {
    let cli = ConvlintCli::parse();
    match cli.run() {
        Ok(()) => {}
        Err(e) => println!("[{sev}] {e}.", sev = Severity::Error),
    }
}
