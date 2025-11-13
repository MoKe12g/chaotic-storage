use crate::models::response::{EntriesCountResponse, MessageResponse};
use crate::models::transaction::Transaction;
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use sqlx::query_as;

#[get("/transactions?<limit>&<page>")]
pub(crate) async fn get_transaction(app_state: &State<api::AppStatePointer>,
                                    limit: Option<i64>,
                                    page: Option<i64>) -> Result<Json<Vec<Transaction>>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let limit = limit.unwrap_or(12);
    let page = page.unwrap_or(0);
    let start = limit * page + 1;
    let end = limit * (page + 1);
    match query_as!(Transaction, "SELECT * FROM transactions WHERE id BETWEEN ?1 AND ?2;", start, end).fetch_all(storage_system.get_database()).await {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" })))
    }
}

#[get("/transactions/<id>")]
pub(crate) async fn get_transaction_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Transaction>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let transaction_from_id = Transaction::from(&storage_system, id).await;
    match transaction_from_id {
        Ok(transaction_from_id) => {
            transaction_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

/// creates entry
#[post("/transactions", data = "<input>")]
pub async fn post_transaction(app_state: &State<api::AppStatePointer>, input: Json<Transaction>) -> Result<Json<Transaction>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    // TODO: Is there a better way than to just discard the given id?
    let input = input.into_inner();
    match Transaction::create(&storage_system, input.allocation_id, input.item_delta, input.date).await {
        Ok(result) => { Ok(Json(result)) }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

/// updates entry
#[patch("/transactions/<id>", data = "<input>")]
pub async fn patch_transaction(app_state: &State<api::AppStatePointer>, id: i64,
                               input: Json<Transaction>) -> Result<Json<Transaction>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let new_value = Transaction { id, allocation_id: input.allocation_id, item_delta: input.item_delta, date: input.date }; // make sure that the id is right inside the struct
    match new_value.update(&storage_system).await {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(new_value)),
        Ok(_) => Err(BadRequest(Json(MessageResponse { message: "No rows updated".into() }))),
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}

#[delete("/transactions/<id>")]
pub async fn delete_transaction(app_state: &State<api::AppStatePointer>, id: i64) -> Result<Json<Transaction>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    match Transaction::from(&storage_system, id).await {
        Ok(result) => {
            match result {
                None => { Err(BadRequest(Json(MessageResponse { message: "Cannot find element".to_string() }))) } // BadRequest(Json(MessageResponse { message: "Cannot find id ".to_owned() + &*id.to_string() })))}
                Some(result2) => {
                    let transaction = result2.clone();
                    match result2.delete(&storage_system).await {
                        Ok(_) => { Ok(Json(transaction)) }
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
#[get("/count/transactions")]
pub async fn count_transaction_entries(app_state: &State<api::AppStatePointer>) -> Result<Json<EntriesCountResponse>, BadRequest<Json<MessageResponse>>> {
    let storage_system = {
        let app_state = app_state.lock().await;
        app_state.get_storage_system().clone()
    };
    let result = query_as!(EntriesCountResponse, "SELECT COUNT(id) AS count, 'transactions' AS 'table' FROM transactions;").fetch_one(storage_system.get_database()).await;
    match result {
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => { Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))) }
    }
}
