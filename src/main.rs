mod cli;
mod pdf;
mod export;
mod error;
mod batch;
mod gui;

use anyhow::Result;
use cli::Cli;
use clap::Parser;
use std::env;

fn main() -> Result<()> {
    // Vérifier si l'utilisateur veut le mode GUI
    let args: Vec<String> = env::args().collect();

    // Si lancé avec --gui ou sans arguments, lancer la GUI
    if args.len() == 1 || args.contains(&"--gui".to_string()) || args.contains(&"gui".to_string()) {
        // Mode GUI
        gui::run().map_err(|e| anyhow::anyhow!("Erreur GUI: {}", e))?;
    } else {
        // Mode CLI
        let cli = Cli::parse();
        cli.execute()?;
    }

    Ok(())
}
