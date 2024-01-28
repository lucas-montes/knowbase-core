use crate::engine::tf;
use crate::file_handlers::{add_file_paths, get_file_paths, remove_file_paths};
use std::path::PathBuf;

pub async fn add_files(paths: Vec<PathBuf>) {
    add_file_paths(paths).await;
}

pub async fn remove_files(paths: Vec<PathBuf>) {
    remove_file_paths(paths).await;
}

pub async fn train() {
    let paths = get_file_paths().await;
    tf(paths).await;
}
