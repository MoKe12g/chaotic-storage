use crate::models::allocations::Allocation;
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
        write!(f, "database inconsistency: inserted allocation not found")
    }
}

impl error::Error for DBInconsistentError {}

impl Allocation {
    pub async fn from(storage_system: &StorageSystem, id: i64) -> Result<Option<Allocation>, Box<dyn Error + Send + Sync>>
    {
        sqlx::query_as!(Allocation,
            "SELECT * from allocations where id == ?1;", id)
            .fetch_optional(storage_system.get_database()).await.map_err(|e| e.into())
    }

    pub async fn insert(storage_system: &StorageSystem, description: String, date_of_entry: NaiveDateTime,
                        can_be_outside: Option<bool>, category_id: i64, storage_box_id: i64) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Self::create(storage_system, description, date_of_entry, can_be_outside, category_id, storage_box_id).await
    }

    pub async fn create(storage_system: &StorageSystem, description: String, date_of_entry: NaiveDateTime,
                        can_be_outside: Option<bool>, category_id: i64, storage_box_id: i64) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!("INSERT INTO allocations (description, date_of_entry, can_be_outside, category_id, storage_box_id) VALUES (?1, ?2, ?3, ?4, ?5);",
        description, date_of_entry, can_be_outside, category_id, storage_box_id).execute(storage_system.get_database()).await?;
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
        Allocation::update_record(storage_system, self.id, &self).await
    }

    pub async fn update_record(storage_system: &StorageSystem, id: i64, allocation: &Allocation) -> Result<SqliteQueryResult, Box<dyn Error + Send + Sync>> {
        match sqlx::query!("UPDATE allocations SET description == ?1, date_of_entry == ?3, can_be_outside == ?4, category_id == ?5, storage_box_id == ?6 WHERE id == ?2;",
            allocation.description, id, allocation.date_of_entry, allocation.can_be_outside, allocation.category_id, allocation.storage_box_id).execute(storage_system.get_database()).await {
            Ok(result) => { Ok(result) }
            Err(err) => { Err(err.into()) }
        }
    }

    pub async fn delete(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult, sqlx::Error> {
        Allocation::delete_record(storage_system, self.id).await
    }

    pub async fn delete_record(storage_system: &StorageSystem, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
        match sqlx::query!("DELETE FROM allocations WHERE id == ?1;", id).execute(storage_system.get_database()).await {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err),
        }
    }
}
