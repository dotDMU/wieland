use crate::item::Item;
use async_trait::async_trait;

#[async_trait]
pub trait ItemRepo: Send + Sync {
    async fn add(&self, item: Item) -> Result<(), String>;
    async fn list(&self) -> Result<Vec<Item>, String>;
}