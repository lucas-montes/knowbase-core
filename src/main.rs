mod cli;
mod cli_handlers;
mod engine;
mod file_handlers;
mod models;

use aromatic::migrate;
use menva::read_default_file;

use cli::Cli;

#[tokio::main]
async fn main() {
    read_default_file();
    migrate("migrations").await;
    Cli::handle().await;
}
