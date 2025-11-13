use crate::webapi::naivedatetime_deserialization::deserialize_datetime;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Transaction {
    pub(crate) id: i64,
    pub(crate) allocation_id: i64,
    pub(crate) item_delta: i64,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub(crate) date: NaiveDateTime,
}
