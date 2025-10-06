use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Transaction {
    pub(crate) id: i64,
    pub(crate) allocation_id: i64,
    pub(crate) item_delta: i64,
    pub(crate) date: NaiveDateTime,
}
