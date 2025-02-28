use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Create a new Git Freedom repository.")]
    Init {
        #[arg(
            short('N'),
            long,
            help = "Name of the repository. (default: Curret repository name)"
        )]
        name: Option<String>,
    },
    // TODO: Make private key to point to a file
    #[command(about = "Update the repository on the Blockchain.")]
    Share {
        #[arg(help = "The private key path of your gitfreedom wallet.")]
        private_key_path: PathBuf,
        #[arg(short('N'), long, help = "Name of the repository.")]
        name: Option<String>,
    },
    #[command(about = "Clone a repository given a public key and the repository name.")]
    Clone {
        #[arg(help = "The full name of the repository. In the format: <name>-<public_key>")]
        full_name: String,
    },
    #[command(about = "Update the seeded repos.")]
    Update {
        #[arg(
            short('A'),
            long,
            help = "Update all the seeding repos.",
            exclusive = true
        )]
        all: bool,
        #[arg(short('N'), long, help = "Name(s) of the repository(ies).")]
        names: Vec<String>,
    },
    #[command(about = "Initialize Git Freedom deamon.")]
    Config {
        #[arg(short('K'), long, help = "The public key of your gitfreedom wallet.")]
        public_key: Option<String>,
    },
    #[command(about = "List all repositories been seeding.")]
    List,
    #[command(about = "Show info about a Git Freedom repository.")]
    Status {
        #[arg(help = "Name of a seeded repository.")]
        name: String,
    },
}
