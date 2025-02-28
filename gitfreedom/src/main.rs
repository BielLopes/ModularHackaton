mod cli;

use std::error::Error;

use clap::Parser;
use gitfreedom::config::configuer::Configuer;
use gitfreedom::errors::Errors::GitFreedomFolderNotFound;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Check if the Git Freedom configuration folder exists
    let exist_config = Configuer::find_gitfreedom_config()?.0;
    if !exist_config && !matches!(args.commands, Commands::Config { .. }) {
        eprintln!("[ERROR]: {GitFreedomFolderNotFound}");
        std::process::exit(1);
    }

    // Run the command
    if let Err(e) = match args.commands {
        Commands::Init { name } => gitfreedom::init::run(name).await,
        Commands::Share {
            private_key_path,
            name,
        } => gitfreedom::share::run(private_key_path, name).await,
        Commands::Clone { full_name } => gitfreedom::clone::run(full_name).await,
        Commands::Config { public_key } => gitfreedom::config::run(public_key),
        _ => Ok(()),
    } {
        eprintln!("[ERROR]: {}", e);
        std::process::exit(1);
    };

    Ok(())
}
