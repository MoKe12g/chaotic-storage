use crate::models::category::Category;
use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;
use sqlx_conditional_queries::conditional_query_as;

#[get("/categories?<limit>&<page>&<id>&<comment>")]
pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>,
                                 id: Option<i64>,
                                 comment: Option<String>) -> Result<Json<Vec<Category>>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };

    // calculate pagination
    let new_page = page.unwrap_or(0);
    let new_limit = limit.unwrap_or(64);
    let offset = new_limit * new_page;

    match conditional_query_as!(Category,
        r#"SELECT *
        FROM categories
        WHERE 1
        {#id}
        {#comment}
        ORDER BY ID ASC
        {#pagination};"#,
        #id = match id.as_ref() {
            Some(_) =>
                "AND id = {id}",
            None => "",
        },
        #comment = match comment.as_ref() {
            Some(_) =>
                "AND comment LIKE '%' || {comment} || '%'",
            None => "",
        },
        #pagination = match limit.as_ref() {
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

#[get("/categories/<id>")]
pub(crate) async fn get_category_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Category>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let categorie_from_id = Category::from(&storage_system, id).await;
    match categorie_from_id {
        Ok(category_from_id) => {
            category_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/categories", data = "<input>")]
pub async fn post_category(app_state: &State<api::AppStatePointer>, input: Json<Category>) -> Result<Json<Category>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    // TODO: Is there a better way than to just discard the given id?
    match Category::create(&storage_system, input.into_inner().comment).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/categories/<id>", data = "<input>")]
pub async fn patch_category(app_state: &State<api::AppStatePointer>, id: i64,
                            input: Json<Category>) -> Result<Json<Category>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let new_value = Category { id, comment: input.comment.clone() }; // make sure that the id is right inside the struct
    match new_value.update(&storage_system).await {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(new_value)),
        Ok(_) => Err(BadRequest(Json(MessageResponse { message: "No rows updated".into() }))),
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/categories/<id>")]
pub async fn delete_category(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<Category>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    match Category::from(&storage_system, id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let category = result2.clone();
                    match result2.delete(&storage_system).await {
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
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'categories' AS 'table' FROM categories;").fetch_one(storage_system.get_database()).await;
    match result
    {
        Err(e) => { Err(BadRequest(Json(MessageResponse { message: e.to_string() }))) }
        Ok(result) => Ok(Json(result))
    }
}
