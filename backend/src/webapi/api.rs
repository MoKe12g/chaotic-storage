use crate::storage_system::storage_system::StorageSystem;
use crate::webapi::{allocations, categories, item_type, storage_boxes, transactions};
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
                    categories::patch_category,
                    categories::delete_category,
                    categories::post_category,
                    categories::count_category_entries,
                    storage_boxes::get_storage_box,
                    storage_boxes::get_storage_box_by_id,
                    storage_boxes::patch_storage_box,
                    storage_boxes::delete_storage_box,
                    storage_boxes::post_storage_box,
                    storage_boxes::count_storage_box_entries,
                    allocations::get_allocation,
                    allocations::get_allocation_by_id,
                    allocations::patch_allocation,
                    allocations::delete_allocation,
                    allocations::post_allocation,
                    allocations::count_allocation_entries,
                    transactions::get_transaction,
                    transactions::get_transaction_by_id,
                    transactions::patch_transaction,
                    transactions::delete_transaction,
                    transactions::post_transaction,
                    transactions::count_transaction_entries,
                    item_type::get_item_type,
                    item_type::get_item_type_by_id,
                    item_type::patch_item_type,
                    item_type::delete_item_type,
                    item_type::post_item_type,
                    item_type::count_item_type_entries,
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

    pub fn get_storage_system(&self) -> &StorageSystem {
        &self.storage_system
    }
}