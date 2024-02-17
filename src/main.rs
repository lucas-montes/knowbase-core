mod cli;
mod engine;
mod file_handlers;
mod markdown_parser;
mod models;

use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::handle().await;
}
