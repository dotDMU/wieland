use domain::item::Item;
use domain::item_contract::ItemContract;

use async_trait::async_trait;
use idb::{Database, DatabaseEvent, Factory, KeyPath, ObjectStoreParams, TransactionMode};
use serde_wasm_bindgen as swb;

const DB_NAME: &str = "eu_app_db";
const DB_VERSION: u32 = 1;
const STORE_ITEMS: &str = "items";

pub struct IdbItemRepo {
    db: Database,
}

impl IdbItemRepo {
    pub async fn new() -> Result<Self, String> {
        let factory = Factory::new().map_err(|e| format!("{:?}", e))?;
        let mut req = factory.open(DB_NAME, Some(DB_VERSION)).map_err(|e| format!("{:?}", e))?;

        req.on_upgrade_needed(|evt| {
            if let Ok(db) = evt.database() {
                if !db.store_names().iter().any(|s| s == STORE_ITEMS) {
                    let mut p = ObjectStoreParams::new();
                    p.key_path(Some(KeyPath::new_single("id")));
                    db.create_object_store(STORE_ITEMS, p).unwrap();
                }
            }
        });

        let db = req.await.map_err(|e| format!("{:?}", e))?;
        Ok(Self { db })
    }
}

#[async_trait(?Send)]
impl ItemContract for IdbItemRepo {
    async fn add(&self, item: Item) -> Result<(), String> {
        let tx = self.db
            .transaction(&[STORE_ITEMS], TransactionMode::ReadWrite)
            .map_err(|e| format!("{:?}", e))?;
        let store = tx.object_store(STORE_ITEMS).unwrap();

        let js = swb::to_value(&item).map_err(|e| format!("{:?}", e))?;
        store.put(&js, None).map_err(|e| format!("{:?}", e))?.await
            .map_err(|e| format!("{:?}", e))?;

        tx.commit().map_err(|e| format!("{:?}", e))?.await
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Item>, String> {
        let tx = self.db
            .transaction(&[STORE_ITEMS], TransactionMode::ReadOnly)
            .map_err(|e| format!("{:?}", e))?;
        let store = tx.object_store(STORE_ITEMS).unwrap();

        let vals = store.get_all(None, None).map_err(|e| format!("{:?}", e))?
            .await.map_err(|e| format!("{:?}", e))?;
        tx.await.map_err(|e| format!("{:?}", e))?;

        let items = vals.into_iter()
            .filter_map(|v| swb::from_value::<Item>(v).ok())
            .collect();
        Ok(items)
    }
}
