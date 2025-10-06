use crate::models::category::Category;
use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::api;
use rocket::State;
use sqlx::Error;

pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>) {
    todo!()
}

pub(crate) async fn get_category_by_id(storage_system: &StorageSystem, id: i64) -> Option<Category> {
    Category::from(storage_system, id).await.ok().flatten()
}

pub(crate) async fn put_category(storage_system: &StorageSystem, category: Category) -> Result<(), Error> {
    todo!()
}
