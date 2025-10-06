use crate::models::response::{CountOfItemsInAllocation, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{get, State};
use sqlx::query_as;

#[get("/allocation_item_count/<allocation_id>")]
pub(crate) async fn get_allocation_by_id(app_state: &State<api::AppStatePointer>, allocation_id: i64) -> Result<Json<CountOfItemsInAllocation>, BadRequest<Json<MessageResponse>>> {
    // TODO: Get all transactions
    // TODO Sum all transactions up
    let app_state = app_state.lock().await;
    match query_as!(CountOfItemsInAllocation, "SELECT sum(item_delta) as 'item_count!', (allocation_id) as 'allocation_id!' from transactions where allocation_id = ?1 LIMIT 1;", allocation_id)
        .fetch_optional(app_state.get_storage_system().get_database()).await {
        Err(err) => Err(BadRequest(Json(MessageResponse { message: err.to_string() + " from backend" }))),
        Ok(transaction) => {
            match transaction {
                Some(transaction) => { Ok(Json(transaction)) }
                None => Err(BadRequest(Json(MessageResponse { message: "No response from server".to_string() })))
            }
        }
    }
}