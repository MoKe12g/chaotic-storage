use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::models::storage_box::StorageBox;
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;

#[get("/storage_boxes?<limit>&<page>")]
pub(crate) async fn get_storage_box(app_state: &State<api::AppStatePointer>,
                                    limit: Option<i64>,
                                    page: Option<i64>) -> Result<Json<Vec<StorageBox>>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let limit = limit.unwrap_or(12);
    let page = page.unwrap_or(0);
    let start = limit * page + 1;
    let end = limit * (page + 1);
    match query_as!(StorageBox, "SELECT * FROM storage_boxes WHERE id BETWEEN ?1 AND ?2;", start, end).fetch_all(app_state.get_storage_system().get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/storage_boxes/<id>")]
pub(crate) async fn get_storage_box_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<StorageBox>> {
    let app_state = app_state.lock().await;
    let storages_box_from_id = StorageBox::from(app_state.get_storage_system(), id).await;
    match storages_box_from_id {
        Ok(storages_box_from_id) => {
            storages_box_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/storage_boxes", data = "<input>")]
pub async fn post_storage_box(app_state: &State<api::AppStatePointer>, input: Json<StorageBox>) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    // TODO: Is there a better way than to just discard the given id?
    match StorageBox::create(app_state.get_storage_system(), input.place.clone(), input.item_type).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/storage_boxes/<id>", data = "<input>")]
pub async fn patch_storage_box(app_state: &State<api::AppStatePointer>, id: i64,
                               input: Json<StorageBox>) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let new_value = StorageBox { id, place: input.place.clone(), item_type: input.item_type }; // make sure that the id is right inside the struct
    match new_value.update(app_state.get_storage_system()).await {
        Ok(_) => { Ok(Json(new_value)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/storage_boxes/<id>")]
pub async fn delete_storage_box(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    match StorageBox::from(app_state.get_storage_system(), id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let storage_box = result2.clone();
                    match result2.delete(app_state.get_storage_system()).await {
                        Ok(_) => { Ok(Json(storage_box)) }
                        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
                    }
                }
            }
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

// misc
// TODO: Anzahl von erstellten Kategorien
#[get("/count/storage_boxes")]
pub async fn count_storage_box_entries(app_state: &State<api::AppStatePointer>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'storage_boxes' AS 'table' FROM storage_boxes;").fetch_optional(app_state.lock().await.get_storage_system().get_database()).await;
    match result {
        Ok(result) => {
            match result
            {
                None => { Err(BadRequest(Json(MessageResponse { message: "Backend couldn't answer the request".to_string() }))) }
                Some(result) => Ok(Json(result))
            }
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}
