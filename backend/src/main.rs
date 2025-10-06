use std::str::FromStr;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

mod storage_system;
mod webapi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_filepath = "database.sqlite";
    let _database_url = format!("{}{}", "sqlite://",database_filepath);

    let database = create_database_connection(_database_url).await?;

    // create storage_system
    let storage_system = storage_system::storage_system::StorageSystem::new(database);

    // creeate webapi with storage_system
    let api = webapi::api::API::new(storage_system);

    api.run().await.expect("TODO: panic message");

    println!("Shutting Down.");
    Ok(())
}

async fn create_database_connection(database_url: String) -> Result<Pool<Sqlite>, sqlx::Error> {
    let sqlite_options = SqliteConnectOptions::from_str(&database_url)?
        .foreign_keys(true)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let database = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(sqlite_options)
        .await?;

        sqlx::migrate!().run(&database).await?;
    Ok(database)
}
