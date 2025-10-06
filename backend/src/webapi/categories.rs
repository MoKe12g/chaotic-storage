use crate::models::category::Category;
use crate::webapi::api;
use rocket::serde::json::Json;
use rocket::{get, put, State};

#[get("/categories?<limit>&<page>")]
pub(crate) async fn get_category(app_state: &State<api::AppStatePointer>,
                                 limit: Option<i64>,
                                 page: Option<i64>) {
    todo!()
}

#[get("/categories/<id>")]
pub(crate) async fn get_category_by_id(app_state: &State<api::AppStatePointer>, id: i64) -> Option<Json<Category>> {
    let app_state = app_state.lock().await;
    let user_from_id = Category::from(app_state.get_database(), id).await;
    match user_from_id {
        Ok(user_from_id) => {
            user_from_id.map(Json)
        }
        Err(_) => { None }
    }
}

#[put("/categories")]
pub(crate) async fn put_category(app_state: &State<api::AppStatePointer>) -> Option<Json<Category>> {
    todo!()
}
