use crate::models::item_type::ItemType;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;
use sqlx_conditional_queries::conditional_query_as;

#[get("/item_types?<limit>&<page>&<id>&<storage_property>")]
pub(crate) async fn get_item_type(app_state: &State<api::AppStatePointer>,
                                  limit: Option<i64>,
                                  page: Option<i64>,
                                  id: Option<i64>,
                                  storage_property: Option<String>) -> Result<Json<Vec<ItemType>>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };

    // calculate pagination
    let new_page = page.unwrap_or(0);
    let new_limit = limit.unwrap_or(64);
    let offset = new_limit * new_page;

    match conditional_query_as!(ItemType,
        r#"SELECT *
        FROM item_types
        WHERE 1
        {#id}
        {#storage_property}
        ORDER BY ID ASC
        {#pagination};"#,
        #id = match id.as_ref() {
            Some(_) =>
                "AND id = {id}",
            None => "",
        },
        #storage_property = match storage_property.as_ref() {
            Some(_) =>
                "AND storage_property = {storage_property}",
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

#[get("/item_types/<id>")]
pub(crate) async fn get_item_type_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<ItemType>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let item_types_from_id = ItemType::from(&storage_system, id).await;
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
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    // TODO: Is there a better way than to just discard the given id?
    match ItemType::create(&storage_system, input.into_inner().storage_property).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/item_types/<id>", data = "<input>")]
pub async fn patch_item_type(app_state: &State<api::AppStatePointer>, id: i64,
                             input: Json<ItemType>) -> Result<Json<ItemType>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let new_value = ItemType { id, storage_property: input.storage_property.clone() }; // make sure that the id is right inside the struct
    match new_value.update(&storage_system).await {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(new_value)),
        Ok(_) => Err(BadRequest(Json(MessageResponse { message: "No rows updated".into() }))),
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/item_types/<id>")]
pub async fn delete_item_type(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<ItemType>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    match ItemType::from(&storage_system, id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let item_type = result2.clone();
                    match result2.delete(&storage_system).await {
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
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'item_types' AS 'table' FROM item_types;").fetch_one(storage_system.get_database()).await;
    match result {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}
