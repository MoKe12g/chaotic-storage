use rocket::{get, put, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use sqlx::{Error, FromRow};
use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::api;

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Kategorie{
    id: i64,
    comment: String,
}

impl Kategorie {
    pub async fn from(storage_system: &StorageSystem, id: i32) -> Result<Option<Kategorie>, Error>
    {
        let result = sqlx::query_as!(Kategorie,
            "SELECT * from kategorien where id = ?1;", id)
            .fetch_all(storage_system.get_database()).await;
        match result{
            Ok(result) => {
                match result.first(){
                    None => {Ok(None)}
                    Some(kategorie) => {
                        Ok(Some(kategorie.clone()))
                    }
                }
            }
            Err(err) => {Err(err)}
        }
    }
}

#[get("/kategorien?<limit>&<page>")]
pub(crate) async fn get_kategorie(app_state: &State<api::AppStatePointer>,
                                   limit: Option<i64>,
                                   page: Option<i64>){
    todo!()
}

#[get("/kategorien/<id>")]
pub(crate) async fn get_kategorie_by_id(app_state: &State<api::AppStatePointer>, id: i32) -> Option<Json<Kategorie>> {
        let app_state = app_state.lock().await;
    let user_from_id = Kategorie::from(app_state.get_database(), id).await;
    match user_from_id{
        Ok(user_from_id) => {
            match user_from_id {
                Some(user_from_id) => {Some(Json(user_from_id))}
                None => {None}
            }
        }
        Err(_) => {None}
    }
}

#[put("/kategories")]
pub(crate) async fn put_kategorien(app_state: &State<api::AppStatePointer>) -> Option<Json<Kategorie>> {
    todo!()
}
