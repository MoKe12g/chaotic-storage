use crate::webapi::naivedatetime_deserialization::deserialize_datetime;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Allocation {
    pub(crate) id: i64,
    pub(crate) description: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub(crate) date_of_entry: NaiveDateTime,
    pub(crate) can_be_outside: Option<bool>,
    pub(crate) category_id: i64,
    pub(crate) storage_box_id: i64,
}
