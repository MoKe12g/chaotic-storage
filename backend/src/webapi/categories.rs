use crate::models::category::Category;
use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::api;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, put, State};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, FromRow};
use std::{error, fmt};

#[derive(Debug, Clone)]
struct DBInconsistentError;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

impl fmt::Display for DBInconsistentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "database inconsistency: inserted category not found")
    }
}

impl error::Error for DBInconsistentError {}

impl Category {
    pub async fn from(storage_system: &StorageSystem, id: i64) -> Result<Option<Category>>
    {
        sqlx::query_as!(Category,
            "SELECT * from categories where id == ?1;", id)
            .fetch_optional(storage_system.get_database()).await.map_err(|e| e.into())
    }

    pub async fn insert(storage_system: StorageSystem, comment: String) -> Result<Self> {
        Self::create(storage_system, comment).await
    }

    pub async fn create(storage_system: StorageSystem, comment: String) -> Result<Self> {
        let result = sqlx::query!("INSERT INTO categories (comment) VALUES (?1);", comment).execute(storage_system.get_database()).await?;
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

    async fn update(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult> {
        match sqlx::query!("UPDATE main.categories SET comment == ?1 WHERE id == ?2;", self.comment, self.id).execute(storage_system.get_database()).await {
            Ok(result) => { Ok(result) }
            Err(err) => { Err(err.into()) }
        }
    }

    async fn delete(&self, storage_system: &StorageSystem) -> Result<SqliteQueryResult> {
        match sqlx::query!("DELETE FROM categories WHERE id == ?1;", self.id).execute(storage_system.get_database()).await {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.into()),
        }
    }
}

#[get("/kategorien?<limit>&<page>")]
pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>) {
    todo!()
}

#[get("/kategorien/<id>")]
pub(crate) async fn get_category_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Category>> {
    let app_state = app_state.lock().await;
    let user_from_id = Category::from(app_state.get_database(), id).await;
    match user_from_id {
        Ok(user_from_id) => {
            user_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

#[put("/kategories")]
pub(crate) async fn put_category(app_state: &State<api::AppStatePointer>) -> Option<Json<Category>> {
    todo!()
}
