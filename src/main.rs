mod cli;
mod file_handler;
mod models;
mod engine;

use aromatic::migrate;
use menva::read_env_file;

use cli::Cli;

#[tokio::main]
async fn main() {
    read_env_file();
    migrate("migrations").await;
    Cli::handle().await;
}
