mod cli;
mod engine;
mod file_handlers;
mod markdown_parser;
mod models;

use cli::Cli;

use aromatic::migrate;
use menva::read_default_file;

#[tokio::main]
async fn main() {
    read_default_file();
    migrate("migrations").await;
    Cli::handle().await;
}
