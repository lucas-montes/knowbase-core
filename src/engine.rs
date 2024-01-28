use crate::file_handlers::get_word_count;
use crate::models::{File, FileWordRelation, Manager, Word, connect};
use std::{fs, path::PathBuf};

pub async fn tf(paths: Vec<PathBuf>) {
    let connection = connect().await;
    for filepath in paths {
        let word_count = get_word_count(&filepath);
        let fullpath = fs::canonicalize(filepath).unwrap();
        let file = File::new(fullpath.to_string_lossy().to_string())
            .get_or_create(&connection)
            .await;

        for (key, value) in word_count.iter() {
            let word = Word::new(key.to_string()).get_or_create(&connection).await;
            let _ = FileWordRelation::new(word.id, file.id, *value)
                .create_or_update(&connection)
                .await;
        }
    }
}
