mod cli;
mod pdf;
mod export;
mod error;
mod batch;

use anyhow::Result;
use cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()?;
    Ok(())
}
