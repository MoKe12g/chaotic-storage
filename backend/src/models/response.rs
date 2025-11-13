use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;

#[derive(Debug, Responder, Deserialize, Serialize)]
pub struct MessageResponse {
    /// This is a message from the server.
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntriesCountResponse {
    pub table: String,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CountOfItemsInAllocation {
    pub allocation_id: i64,
    pub item_count: i64,
}

