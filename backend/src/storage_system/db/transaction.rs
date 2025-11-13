use crate::models::transaction::Transaction;
use crate::storage_system::storage_system::StorageSystem;
use chrono::NaiveDateTime;
use sqlx::sqlite::SqliteQueryResult;
use std::error::Error;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct DBInconsistentError;

//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

impl fmt::Display for DBInconsistentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "database inconsistency: inserted transaction not found")
    }
}

impl error::Error for DBInconsistentError {}

impl Transaction {
    pub async fn from(storage_system: &StorageSystem, id: i64) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync>>
    {
        sqlx::query_as!(Transaction,
            "SELECT * from transactions where id == ?1;", id)
            .fetch_optional(storage_system.get_database()).await.map_err(|e| e.into())
    }

    pub async fn insert(storage_system: &StorageSystem, allocation_id: i64, item_delta: i64, date: NaiveDateTime) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Self::create(storage_system, allocation_id, item_delta, date).await
    }

    pub async fn create(storage_system: &StorageSystem, allocation_id: i64, item_delta: i64, date: NaiveDateTime) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!("INSERT INTO transactions (allocation_id, item_delta, date) VALUES (?1, ?2, ?3);",
        allocation_id, item_delta, date).execute(storage_system.get_database()).await?;
        let id = result.last_insert_rowid();
        match Self::from(storage_system, id).await {
            Ok(result) => {
                match result {
                    Some(result) => { Ok(result) }
                    None => { Err(DBInconsistentError.into()) }
                }
            }
            Err(err) => { Err(err) }
        }
    }

    pub async fn update(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult, Box<dyn Error + Send + Sync>> {
        Transaction::update_record(storage_system, self.id, &self).await
    }

    pub async fn update_record(storage_system: &StorageSystem, id: i64, transaction: &Transaction) -> Result<SqliteQueryResult, Box<dyn Error + Send + Sync>> {
        match sqlx::query!("UPDATE main.transactions SET allocation_id == ?1, item_delta = ?3, date = ?4 WHERE id == ?2;",
            transaction.allocation_id, id, transaction.item_delta, transaction.date).execute(storage_system.get_database()).await {
            Ok(result) => { Ok(result) }
            Err(err) => { Err(err.into()) }
        }
    }

    pub async fn delete(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult, sqlx::Error> {
        Transaction::delete_record(storage_system, self.id).await
    }

    pub async fn delete_record(storage_system: &StorageSystem, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
        match sqlx::query!("DELETE FROM transactions WHERE id == ?1;", id).execute(storage_system.get_database()).await {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err),
        }
    }
}
