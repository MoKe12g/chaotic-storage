use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub(crate) struct Category {
    pub(crate) id: i64,
    pub(crate) comment: String,
}
