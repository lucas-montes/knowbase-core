use crate::engine::process;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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
            Commands::Add { path } => {
                todo!("f");
            }
            Commands::Similarity {
                label,
                item,
                training,
            } => {
                    println!("starting process");
                process(label, item, training).await;
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    #[command(arg_required_else_help = true)]
    Similarity {
        #[arg(short, long)]
        label: String,
        #[arg(short, long)]
        item: String,
        #[arg(short, long)]
        training: PathBuf,
    },
}
