use clap::Parser;
use ssmync::{self, CommandLineArgs};
use std::process;

#[tokio::main]
async fn main() {
    let cli_args = CommandLineArgs::parse();
    if let Err(e) = ssmync::run(cli_args).await {
        eprint!("Application error {}", e);
        process::exit(1);
    }
}
