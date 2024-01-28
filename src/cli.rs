use crate::cli_handlers::{add_files, remove_files, train};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "kb")]
#[command(about = "A knowbase CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn handle() {
        let args = Cli::parse();

        match args.command {
            Commands::Server => {
                todo!("im the server and im not ready");
            }
            Commands::Remove { paths } => {
                remove_files(paths).await;
            }
            Commands::Add { paths } => {
                add_files(paths).await;
            }
            Commands::Train => {
                train().await;
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Train,
    #[command(arg_required_else_help = true)]
    Remove {
        /// Stuff to remove
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    Server,
}
