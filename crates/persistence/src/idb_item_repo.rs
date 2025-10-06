use domain::item_contract::ItemContract;

use async_trait::async_trait;
use idb::Database;
use domain::item::Item;
use domain::wie_error::WieError;

const DB_NAME: &str = "eu_app_db";
const DB_VERSION: u32 = 1;
const STORE_ITEMS: &str = "items";

pub struct IdbItemRepo {
    db: Database,
}

impl IdbItemRepo {
    pub async fn new() -> Result<IdbItemRepo, WieError> {
        todo!()
    }

}

#[async_trait(?Send)]
impl ItemContract for IdbItemRepo {
    async fn create(&self, item: Item) -> Result<Item, WieError> {
        todo!()
    }
    async fn get(&self, id: &str) -> Result<Option<Item>, WieError> {
        todo!()
    }
    async fn update(&self, item: Item) -> Result<Item, WieError> {
        todo!()
    }
    async fn list(&self) -> Result<Vec<Item>, WieError> {
        todo!()
    }
    async fn delete(&self, id: &str) -> Result<bool, WieError> {
        todo!()
    }
}
