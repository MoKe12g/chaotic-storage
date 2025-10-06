use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct ItemType {
    pub(crate) id: i64,
    pub(crate) storage_property: String,
}
