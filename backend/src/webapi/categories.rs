use crate::models::category::Category;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;

#[get("/categories?<limit>&<page>")]
pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>) -> Result<Json<Vec<Category>>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let limit = limit.unwrap_or(12);
    let page = page.unwrap_or(1);
    let start = limit * page;
    let end = limit * (page + 1) - 1;
    match query_as!(Category, "SELECT * FROM categories WHERE id BETWEEN ?1 AND ?2;", start, end).fetch_all(app_state.get_storage_system().get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/categories/<id>")]
pub(crate) async fn get_category_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Category>> {
    let app_state = app_state.lock().await;
    let categorie_from_id = Category::from(app_state.get_storage_system(), id).await;
    match categorie_from_id {
        Ok(categorie_from_id) => {
            categorie_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/categories", data = "<input>")]
pub async fn post_category(app_state: &State<api::AppStatePointer>, input: Json<Category>) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    // TODO: Is there a better way than to just discard the given id?
    match Category::create(app_state.get_storage_system(), input.into_inner().comment).await {
        Ok(result) => { Ok(Json(result.id.to_string())) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/categories/<id>", data = "<input>")]
pub async fn patch_category(app_state: &State<api::AppStatePointer>, id: i64,
                            input: Json<Category>) -> Result<Json<Category>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    let new_value = Category { id, comment: input.comment.clone() }; // make sure that the id is right inside the struct
    match new_value.update(app_state.get_storage_system()).await {
        Ok(_) => { Ok(Json(new_value)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/categories/<id>")]
pub async fn delete_category(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<Category>, BadRequest<Json<MessageResponse>>> {
    let app_state = app_state.lock().await;
    match Category::from(app_state.get_storage_system(), id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let category = result2.clone();
                    match result2.delete(app_state.get_storage_system()).await {
                        Ok(_) => { Ok(Json(category)) }
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
#[get("/count/categories")]
pub async fn count_category_entries(app_state: &State<api::AppStatePointer>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'categories' AS 'table' FROM categories;").fetch_optional(app_state.lock().await.get_storage_system().get_database()).await;
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
