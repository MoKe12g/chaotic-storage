use crate::models::allocations::Allocation;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;
use sqlx_conditional_queries::conditional_query_as;

#[get("/allocations?<limit>&<page>&<storage_box_id>&<can_be_outside>&<category_id>&<description>")]
pub(crate) async fn get_allocation(app_state: &State<api::AppState>,
                                   limit: Option<i64>,
                                   page: Option<i64>,
                                   storage_box_id: Option<i64>,
                                   can_be_outside: Option<bool>,
                                   category_id: Option<i64>,
                                   description: Option<String>) -> Result<Json<Vec<Allocation>>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };

    // calculate pagination
    let new_page = page.unwrap_or(0);
    let new_limit = limit.unwrap_or(64);
    let start = new_limit * new_page + 1;
    let end = new_limit * (new_page + 1);

    match conditional_query_as!(Allocation,
        r#"SELECT *
        FROM allocations
        WHERE 1
        {#storage_box_id}
        {#can_be_outside}
        {#category_id}
        {#description}
        {#pagination}
        ORDER BY ID ASC;"#,
        #storage_box_id = match storage_box_id {
            Some(_) =>
            "AND storage_box_id = {storage_box_id}",
            None => "",
        },
        #can_be_outside = match can_be_outside {
            Some(_) =>
            "AND can_be_outside = {can_be_outside}",
            None => "",
        },
        #category_id = match category_id {
            Some(_) =>
            "AND category_id = {category_id}",
            None => "",
        },
        #description = match description.as_ref() {
            Some(_) =>
            "AND description = {description}",
            None => "",
        },
        #pagination = match limit {
            Some(_) =>
                "AND id BETWEEN {start} AND {end}",
            None => "",
        },
    ).fetch_all(storage_system.get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/allocations/<id>")]
pub(crate) async fn get_allocation_by_id(app_state: &State<api::AppState>, id: i64) -> Option<Json<Allocation>> {
    let storage_system = app_state.get_storage_system();
    let allocation_from_id = Allocation::from(storage_system, id).await;
    match allocation_from_id {
        Ok(allocation_from_id) => {
            allocation_from_id.map(Json)
        }
        // TODO: Log errors in all files
        Err(_) => None
    }
}

/// creates entry
#[post("/allocations", data = "<input>")]
pub async fn post_allocation(app_state: &State<api::AppState>, input: Json<Allocation>) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    // TODO: Is there a better way than to just discard the given id?
    let input = input.into_inner();
    match Allocation::create(storage_system, input.description, input.date_of_entry, input.can_be_outside, input.category_id, input.storage_box_id).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/allocations/<id>", data = "<input>")]
pub async fn patch_allocation(app_state: &State<api::AppState>, id: i64,
                              input: Json<Allocation>) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    let new_value = Allocation { id, description: input.description.clone(), date_of_entry: input.date_of_entry, can_be_outside: input.can_be_outside, category_id: input.category_id, storage_box_id: input.storage_box_id }; // make sure that the id is right inside the struct
    match new_value.update(&storage_system).await {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(new_value)),
        Ok(_) => Err(BadRequest(Json(MessageResponse { message: "No rows updated".into() }))),
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/allocations/<id>")]
pub async fn delete_allocation(app_state: &State<api::AppState>, id: i64) -> Result<Json<Allocation>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    match Allocation::from(storage_system, id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let allocation = result2.clone();
                    match result2.delete(&storage_system).await {
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
pub async fn count_allocation_entries(app_state: &State<api::AppState>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let storage_system = app_state.get_storage_system();
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'allocations' AS 'table' FROM allocations;").fetch_one(storage_system.get_database()).await;
    match result {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}
