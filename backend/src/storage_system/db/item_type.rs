use crate::models::item_type::ItemType;
use crate::storage_system::storage_system::StorageSystem;
use sqlx::sqlite::SqliteQueryResult;
use std::error::Error;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct DBInconsistentError;

//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

impl fmt::Display for DBInconsistentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "database inconsistency: inserted item_type not found")
    }
}

impl error::Error for DBInconsistentError {}

impl ItemType {
    pub async fn from(storage_system: &StorageSystem, id: i64) -> Result<Option<ItemType>, Box<dyn Error + Send + Sync>>
    {
        sqlx::query_as!(ItemType,
            "SELECT * from item_types where id == ?1;", id)
            .fetch_optional(storage_system.get_database()).await.map_err(|e| e.into())
    }

    pub async fn insert(storage_system: &StorageSystem, storage_property: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Self::create(storage_system, storage_property).await
    }

    pub async fn create(storage_system: &StorageSystem, storage_property: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!("INSERT INTO item_types (storage_property) VALUES (?1);", storage_property).execute(storage_system.get_database()).await?;
        let id = result.last_insert_rowid();
        match Self::from(&storage_system, id).await {
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
        ItemType::update_record(storage_system, self.id, &self).await
    }

    pub async fn update_record(storage_system: &StorageSystem, id: i64, item_type: &ItemType) -> Result<SqliteQueryResult, Box<dyn Error + Send + Sync>> {
        match sqlx::query!("UPDATE main.item_types SET storage_property == ?1 WHERE id == ?2;", item_type.storage_property, id).execute(storage_system.get_database()).await {
            Ok(result) => { Ok(result) }
            Err(err) => { Err(err.into()) }
        }
    }

    pub async fn delete(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult, sqlx::Error> {
        ItemType::delete_record(storage_system, self.id).await
    }

    pub async fn delete_record(storage_system: &StorageSystem, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
        match sqlx::query!("DELETE FROM item_types WHERE id == ?1;", id).execute(storage_system.get_database()).await {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err),
        }
    }
}
