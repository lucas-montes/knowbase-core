mod cli;
mod engine;
mod file_handlers;
mod markdown_parser;
mod models;

use cli::Cli;

use aromatic::migrate;
use menva::read_env_file;

#[tokio::main]
async fn main() {
    read_env_file("/home/lucas/Projects/knowbase/knowbase-core/.env");
    migrate("migrations").await;
    Cli::handle().await;
}
