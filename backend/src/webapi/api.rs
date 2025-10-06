use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::categories;
use rocket::{routes, Error, Ignite, Rocket};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct API
{
    storage_system: StorageSystem,
}

impl API {
    pub fn new(storage_system: StorageSystem) -> API {
        API { storage_system }
    }

    pub(crate) async fn run(&self) -> anyhow::Result<Rocket<Ignite>, Error> {
        rocket::build()
            .manage(AppState::new(self.storage_system.clone())
            )
            .mount(
                "/",
                routes![
                        // TODO: Routes,
                    categories::get_category,
                    categories::get_category_by_id,
                    categories::put_category,
                    ],
            )
            .launch().await
    }
}

pub struct AppState {
    storage_system: StorageSystem,
}

// Could be changed into State<StorageSystem> if no mutable value will be used
pub type AppStatePointer = Arc<Mutex<AppState>>;

impl AppState {
    fn new(storage_system: StorageSystem) -> AppStatePointer {
        let new_app_state = AppState {
            storage_system
        };
        Arc::new(Mutex::new(new_app_state))
    }

    pub fn get_database(&self) -> &StorageSystem {
        &self.storage_system
    }
}