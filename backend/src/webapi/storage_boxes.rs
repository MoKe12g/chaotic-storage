use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::models::storage_box::StorageBox;
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;
use sqlx_conditional_queries::conditional_query_as;

#[get("/storage_boxes?<limit>&<page>&<id>&<place>&<item_type>")]
pub(crate) async fn get_storage_box(app_state: &State<api::AppState>,
                                    limit: Option<i64>,
                                    page: Option<i64>,
                                    id: Option<i64>,
                                    place: Option<String>,
                                    item_type: Option<i64>) -> Result<Json<Vec<StorageBox>>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();

    // calculate pagination
    let new_page = page.unwrap_or(0);
    let new_limit = limit.unwrap_or(64);
    let offset = new_limit * new_page;

    match conditional_query_as!(StorageBox,
        r#"SELECT *
        FROM storage_boxes
        WHERE 1
        {#id}
        {#place}
        {#item_type}
        ORDER BY id ASC
        {#pagination};"#,
        #id = match id.as_ref() {
            Some(_) =>
                "AND id = {id}",
            None => "",
        },
        #place = match place.as_ref() {
            Some(_) =>
                "AND place LIKE '%' || {place} || '%'",
            None => "",
        },
        #item_type = match item_type.as_ref() {
            Some(_) =>
                "AND item_type = {item_type}",
            None => "",
        },
        #pagination = match limit {
            Some(_) =>
                "LIMIT {new_limit} OFFSET {offset}",
            None => "",
        },
    ).fetch_all(storage_system.get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/storage_boxes/<id>")]
pub(crate) async fn get_storage_box_by_id(app_state: &State<api::AppState>, id: i64) -> Option<Json<StorageBox>> {
    let storage_system = app_state.get_storage_system();
    let storages_box_from_id = StorageBox::from(storage_system, id).await;
    match storages_box_from_id {
        Ok(storages_box_from_id) => {
            storages_box_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/storage_boxes", data = "<input>")]
pub async fn post_storage_box(app_state: &State<api::AppState>, input: Json<StorageBox>) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    // TODO: Is there a better way than to just discard the given id?
    match StorageBox::create(storage_system, input.place.clone(), input.item_type).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/storage_boxes/<id>", data = "<input>")]
pub async fn patch_storage_box(app_state: &State<api::AppState>, id: i64,
                               input: Json<StorageBox>) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    let new_value = StorageBox { id, place: input.place.clone(), item_type: input.item_type }; // make sure that the id is right inside the struct
    match new_value.update(&storage_system).await {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(new_value)),
        Ok(_) => Err(BadRequest(Json(MessageResponse { message: "No rows updated".into() }))),
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/storage_boxes/<id>")]
pub async fn delete_storage_box(app_state: &State<api::AppState>, id: i64) -> Result<Json<StorageBox>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    match StorageBox::from(storage_system, id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let storage_box = result2.clone();
                    match result2.delete(&storage_system).await {
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
#[get("/count/storage_boxes")]
pub async fn count_storage_box_entries(app_state: &State<api::AppState>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'storage_boxes' AS 'table' FROM storage_boxes;").fetch_one(storage_system.get_database()).await;
    match result {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}
