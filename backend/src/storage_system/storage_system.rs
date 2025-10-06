use sqlx::{Pool, Sqlite};

#[derive(Debug,Clone)]
pub struct StorageSystem{
    database: Pool<Sqlite>,
}

impl StorageSystem {
    pub(crate) fn get_database(&self) -> &Pool<Sqlite> {
        &self.database
    }
}

impl StorageSystem
{
    pub fn new(database: Pool<Sqlite>) -> StorageSystem{
        StorageSystem {
            database,
        }
    }
}