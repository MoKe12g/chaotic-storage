use crate::models::response::{CountOfItemsInAllocation, MessageResponse};
use crate::webapi::api;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{get, State};
use sqlx::query_as;

#[get("/allocation_item_count/<allocation_id>")]
pub(crate) async fn sum_transaction_items_for_allocation(app_state: &State<api::AppState>, allocation_id: i64) -> Result<Json<CountOfItemsInAllocation>, BadRequest<Json<MessageResponse>>> {
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