use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::categories;
use rocket::futures::lock::Mutex;
use rocket::{routes, Error, Ignite, Rocket};
use std::sync::Arc;

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
            .manage(AppState::new(self.storage_system.clone()) // TODO: Is clone the best way here?
            )
            .mount(
                "/",
                routes![
                        // TODO: Routes,
                    categories::get_category,
                    categories::get_kategory_by_id,
                    categories::put_category,
                    ],
            )
            .launch().await
    }
}

pub struct AppState {
    storage_system: StorageSystem,
}

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