use crate::models::allocations::Allocation;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;

#[get("/allocations?<limit>&<page>")]
pub(crate) async fn get_allocation(app_state: &State<api::AppStatePointer>,
                                   limit: Option<i64>,
                                   page: Option<i64>) -> Result<Json<Vec<Allocation>>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let limit = limit.unwrap_or(12);
    let page = page.unwrap_or(0);
    let start = limit * page;
    let end = limit * (page + 1) - 1;
    match query_as!(Allocation, "SELECT * FROM allocations WHERE id BETWEEN ?1 AND ?2;", start, end).fetch_all(app_state.get_storage_system().get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/allocations/<id>")]
pub(crate) async fn get_allocation_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Allocation>> {
    let app_state = app_state.lock().await;
    let allocation_from_id = Allocation::from(app_state.get_storage_system(), id).await;
    match allocation_from_id {
        Ok(allocation_from_id) => {
            allocation_from_id.map(Json)
        }
        Err(_) => None
    }
}

/// creates entry
#[post("/allocations", data = "<input>")]
pub async fn post_allocation(app_state: &State<api::AppStatePointer>, input: Json<Allocation>) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    // TODO: Is there a better way than to just discard the given id?
    let input = input.into_inner();
    match Allocation::create(app_state.get_storage_system(), input.description, input.date_of_entry, input.can_be_outside, input.category_id, input.storage_box_id).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/allocations/<id>", data = "<input>")]
pub async fn patch_allocation(app_state: &State<api::AppStatePointer>, id: i64,
                              input: Json<Allocation>) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let new_value = Allocation { id, description: input.description.clone(), date_of_entry: input.date_of_entry, can_be_outside: input.can_be_outside, category_id: input.category_id, storage_box_id: input.storage_box_id }; // make sure that the id is right inside the struct
    match new_value.update(app_state.get_storage_system()).await {
        Ok(_) => { Ok(Json(new_value)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/allocations/<id>")]
pub async fn delete_allocation(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    match Allocation::from(app_state.get_storage_system(), id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let allocation = result2.clone();
                    match result2.delete(app_state.get_storage_system()).await {
                        Ok(_) => { Ok(Json(allocation)) }
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
#[get("/count/allocations")]
pub async fn count_allocation_entries(app_state: &State<api::AppStatePointer>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'allocations' AS 'table' FROM allocations;").fetch_optional(app_state.lock().await.get_storage_system().get_database()).await;
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
