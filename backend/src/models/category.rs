use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Category {
    pub(crate) id: i64,
    pub(crate) comment: String,
}
