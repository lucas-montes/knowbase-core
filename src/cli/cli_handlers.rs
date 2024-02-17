use crate::{
    engine::tf,
    file_handlers::{add_file_paths, get_file_paths, remove_file_paths},
    markdown_parser::parse_markdown,
};

use std::path::PathBuf;

pub async fn add_files(paths: Vec<PathBuf>) -> i16 {
    add_file_paths(&paths).await;
    // Lets read it twice now. Maybe read it only once and do clever things like adding weights
    // depending on what the text is (Header(H1), Link, etc...).
    parse_markdown(&paths).await;
    0
}

pub async fn remove_files(paths: Vec<PathBuf>) -> i16 {
    remove_file_paths(paths).await;
    0
}

pub async fn train() -> i16 {
    let paths = get_file_paths().await;
    tf(paths).await;
    0
}
