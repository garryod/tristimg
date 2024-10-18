#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use clap::Parser;
use tracing::Level;

/// Tristimg performs binning of event mode data, generating images
#[derive(Debug, Parser)]
struct Cli {
    /// The minimum log level which should be produced
    #[clap(long, env = "LOG_LEVEL", default_value_t=Level::INFO)]
    log_level: Level,
}

fn main() {
    _ = dotenvy::dotenv();
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    tracing::info!("Hello, world!");
}
