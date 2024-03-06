use async_trait::async_trait;
use menva::get_env;
use sqlx::{
    sqlite::{SqliteConnection, SqlitePool, SqlitePoolOptions, SqliteRow},
    FromRow, Row, Sqlite, Transaction,
};
use std::{path::PathBuf, time::Duration};

#[derive(Debug, FromRow)]
pub struct File {
    pub id: u8,
    pub file: String,
}

impl File {
    pub fn new(file: String) -> Self {
        File { id: 0, file }
    }
    pub async fn get_all(connection: &SqlitePool) -> Vec<String> {
        let query = format!("SELECT file FROM {table}", table = Self::table());
        let rows = sqlx::query(&query).fetch_all(connection).await;
        match rows {
            Ok(result) => result.iter().map(|r| r.try_get("file").unwrap()).collect(),
            Err(err) => {
                panic!("executing query get_all {:?}", err);
            }
        }
    }
    pub async fn delete_many(paths: Vec<PathBuf>, connection: &SqlitePool) -> u64 {
        let values = paths
            .iter()
            .map(|p| format!("'{}'", p.to_str().unwrap().to_owned()))
            .collect::<Vec<String>>()
            .join(",");
        let query = format!(
            "DELETE FROM {table} WHERE {fields} IN ({values});",
            table = Self::table(),
            fields = "file",
            values = values
        );
        let transaction = transaction(connection).await;
        execute_query(&query, transaction).await
    }

    pub async fn insert_many(paths: &Vec<PathBuf>, connection: &SqlitePool) -> u64 {
        let values = paths
            .iter()
            .map(|p| format!("('{}')", std::fs::canonicalize(p).unwrap().to_str().unwrap()))
            .collect::<Vec<String>>()
            .join(",");
        let query = format!(
            "INSERT OR IGNORE INTO {table} ('{fields}') VALUES {values};",
            table = Self::table(),
            fields = "file",
            values = values
        );
        let transaction = transaction(connection).await;
        execute_query(&query, transaction).await
    }
}

impl Manager for File {
    fn create_or_update_query(&self) -> String {
        format!(
            "
            INSERT INTO {table} ({fields})
            VALUES({values})
            ON CONFLICT({conflict_fields})
            DO UPDATE SET {update_fields};
        ",
            table = Self::table(),
            fields = "file",
            values = self.file,
            conflict_fields = "file",
            update_fields = "file"
        )
    }

    fn get_or_create_query(&self) -> String {
        format!(
            "INSERT OR IGNORE INTO {table} ({fields}) VALUES ('{values}');
            SELECT id, file FROM {table} WHERE {fields} = '{values}' LIMIT 1;
        ",
            table = Self::table(),
            fields = "file",
            values = self.file
        )
    }
}

#[derive(Debug, FromRow)]
pub struct Word {
    pub id: u8,
    pub word: String,
}

impl Word {
    pub fn new(word: String) -> Self {
        Word { id: 0, word }
    }
}

impl Manager for Word {
    fn create_or_update_query(&self) -> String {
        format!(
            "
        INSERT INTO {table} ({fields})
        VALUES({values})
        ON CONFLICT({conflict_fields})
        DO UPDATE SET {update_fields};
        ",
            table = Self::table(),
            fields = "word",
            values = self.word,
            conflict_fields = "word",
            update_fields = "word",
        )
    }

    fn get_or_create_query(&self) -> String {
        format!(
            "INSERT OR IGNORE INTO {table} ({fields}) VALUES ('{values}');
            SELECT id, word FROM {table} WHERE {fields} = '{values}' LIMIT 1;
        ",
            table = Self::table(),
            fields = "word",
            values = self.word
        )
    }
}

#[derive(Debug, FromRow)]
pub struct FileWordRelation {
    pub id: u8,
    pub word_id: u8,
    pub file_id: u8,
    pub word_count: u8,
}

impl FileWordRelation {
    pub fn new(word_id: u8, file_id: u8, word_count: u8) -> Self {
        FileWordRelation {
            id: 0,
            word_id,
            file_id,
            word_count,
        }
    }
}

#[async_trait]
impl Manager for FileWordRelation {
    fn create_or_update_query(&self) -> String {
        format!(
            "
            INSERT INTO {table} ({fields})
            VALUES ({word_id},{file_id},{word_count})
            ON CONFLICT(word_id, file_id)
            DO UPDATE SET word_count = {word_count};
            SELECT * FROM {table} WHERE word_id = {word_id} and file_id = {file_id} LIMIT 1;
            ",
            table = Self::table(),
            fields = "word_id, file_id, word_count",
            word_id = self.word_id,
            file_id = self.file_id,
            word_count = self.word_count
        )
    }

    fn get_or_create_query(&self) -> String {
        format!(
            "INSERT OR IGNORE INTO {table} ({fields}) VALUES ('{values}');
            SELECT {selected} FROM {table} WHERE {fields} = '{values}' and  LIMIT 1;
        ",
            table = Self::table(),
            fields = "",
            values = "",
            selected = "",
        )
    }
}

#[async_trait]
pub trait Manager
where
    Self: for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin,
{
    fn create_or_update_query(&self) -> String;

    async fn create_or_update(&self, connection: &SqlitePool) {
        let query = self.create_or_update_query();
        let transaction = transaction(connection).await;
        fetch_query::<Self>(query, transaction).await;
    }

    fn get_or_create_query(&self) -> String;

    async fn get_or_create(self, connection: &SqlitePool) -> Self {
        let query = self.get_or_create_query();
        let transaction = transaction(connection).await;
        fetch_query::<Self>(query, transaction).await
    }

    fn table() -> String {
        format!("{}s", Self::struct_to_snake_case())
    }

    fn struct_to_snake_case() -> String {
        let mut result = String::new();

        for (i, c) in Self::entity_name().chars().enumerate() {
            if c.is_ascii_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }

        result
    }

    fn entity_name() -> String {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap()
            .to_string()
    }
}

async fn execute_query(query: &str, mut transaction: Transaction<'_, Sqlite>) -> u64 {
    let row = sqlx::query(query)
        .execute(&mut transaction as &mut SqliteConnection)
        .await;
    match row {
        Ok(result) => {
            commit_transaction(transaction).await;
            result.rows_affected()
        }
        Err(err) => {
            panic!("executing query {:?}", err);
        }
    }
}

async fn fetch_query<'a, T>(query: String, mut transaction: Transaction<'a, Sqlite>) -> T
where
    T: for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin,
{
    let row = sqlx::query_as::<_, T>(&query)
        .fetch_one(&mut transaction as &mut SqliteConnection)
        .await;
    match row {
        Ok(result) => {
            commit_transaction(transaction).await;
            result
        }
        Err(err) => {
            panic!("executing query {:?}", err);
        }
    }
}

async fn commit_transaction(transaction: Transaction<'_, Sqlite>) {
    match transaction.commit().await {
        Ok(_) => {}
        Err(err) => {
            panic!("transaction commit error: {:?}", err);
        }
    }
}

async fn transaction<'a>(connection: &SqlitePool) -> Transaction<'a, Sqlite> {
    match connection.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            panic!("transaction error launching: {:?}", err);
        }
    }
}

pub async fn connect() -> SqlitePool {
    let options = SqlitePoolOptions::new()
        .max_connections(20)
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(3600));
    match options.connect(&get_env("DATABASE_URL")).await {
        Ok(db) => db,
        Err(e) => panic!(" oupsi in the connection {}", e),
    }
}
