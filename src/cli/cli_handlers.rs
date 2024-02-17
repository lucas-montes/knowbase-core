use crate::{
    engine::tf,
    file_handlers::{add_file_paths, get_file_paths, remove_file_paths},
};

use std::path::PathBuf;

pub async fn add_files(paths: Vec<PathBuf>) -> i16 {
    add_file_paths(paths).await;
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
