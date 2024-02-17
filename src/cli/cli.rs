use crate::cli::cli_handlers::{add_files, remove_files, train};
use lsp::run_server;
use std::io;
use std::path::PathBuf;

use todo::utils::cli::ToDoCli;

use clap::{Parser, Subcommand};
use clap::CommandFactory;
use clap_complete::{generate, Shell};

#[derive(Debug, Parser)]
#[command(name = "kb")]
#[command(about = "A knowbase CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn handle() -> i16 {
        let args = Cli::parse();

        match args.command {
            Commands::Todo(args) => args.handle_args().await,
            Commands::Remove { paths } => remove_files(paths).await,
            Commands::Add { paths } => add_files(paths).await,
            // Move those three below into a separated struct. something like tools for Todo
            Commands::Server => todo!("im the server and im not ready"),
            Commands::Train => train().await,
            Commands::Lsp => match run_server() {
                Ok(_) => 0,
                Err(_) => 1,
            },
            Commands::Completions { shell } => run(shell),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Todo(ToDoCli),
    Train,
    Lsp,
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

    Completions {
        shell: Shell,
    },
}
fn run(shell: Shell) -> i16 {
    generate(shell, &mut Cli::command(), "sqlx", &mut io::stdout());
    0
}
