use crate::models::mod_category::Category;
use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::api;
use rocket::State;
use sqlx::Error;

pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>) {
    todo!()
}

pub(crate) async fn get_category_by_id(storage_system: &StorageSystem, id: i32) -> Option<Category> {
    let user_from_id = Category::from(storage_system, id).await;
    match user_from_id {
        Ok(user_from_id) => {
            match user_from_id {
                Some(user_from_id) => { Some(user_from_id) }
                None => { None }
            }
        }
        Err(_) => { None }
    }
}

pub(crate) async fn put_category(storage_system: &StorageSystem, category: Category) -> Result<(), Error> {
    todo!()
}
