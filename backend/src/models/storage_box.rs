use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct StorageBox {
    pub(crate) id: i64,
    pub(crate) place: String,
    pub(crate) item_type: i64,
}
