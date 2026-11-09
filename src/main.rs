mod cli;
mod cmd;
mod util;

use clap::Parser;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
