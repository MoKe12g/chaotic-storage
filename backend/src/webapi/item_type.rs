use crate::models::item_type::ItemType;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;

#[get("/item_types?<limit>&<page>")]
pub(crate) async fn get_item_type(app_state: &State<api::AppStatePointer>,
                                  limit: Option<i64>,
                                  page: Option<i64>) -> Result<Json<Vec<ItemType>>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let limit = limit.unwrap_or(12);
    let page = page.unwrap_or(0);
    let start = limit * page + 1;
    let end = limit * (page + 1);
    match query_as!(ItemType, "SELECT * FROM item_types WHERE id BETWEEN ?1 AND ?2;", start, end).fetch_all(app_state.get_storage_system().get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/item_types/<id>")]
pub(crate) async fn get_item_type_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<ItemType>> {
    let app_state = app_state.lock().await;
    let item_types_from_id = ItemType::from(app_state.get_storage_system(), id).await;
    match item_types_from_id {
        Ok(item_types_from_id) => {
            item_types_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/item_types", data = "<input>")]
pub async fn post_item_type(app_state: &State<api::AppStatePointer>, input: Json<ItemType>) -> Result<Json<ItemType>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    // TODO: Is there a better way than to just discard the given id?
    match ItemType::create(app_state.get_storage_system(), input.into_inner().storage_property).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/item_types/<id>", data = "<input>")]
pub async fn patch_item_type(app_state: &State<api::AppStatePointer>, id: i64,
                             input: Json<ItemType>) -> Result<Json<ItemType>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let new_value = ItemType { id, storage_property: input.storage_property.clone() }; // make sure that the id is right inside the struct
    match new_value.update(app_state.get_storage_system()).await {
        Ok(_) => { Ok(Json(new_value)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/item_types/<id>")]
pub async fn delete_item_type(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<ItemType>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    match ItemType::from(app_state.get_storage_system(), id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let item_type = result2.clone();
                    match result2.delete(app_state.get_storage_system()).await {
                        Ok(_) => { Ok(Json(item_type)) }
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
#[get("/count/item_types")]
pub async fn count_item_type_entries(app_state: &State<api::AppStatePointer>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'item_types' AS 'table' FROM item_types;").fetch_optional(app_state.lock().await.get_storage_system().get_database()).await;
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
