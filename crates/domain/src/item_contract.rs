use async_trait::async_trait;
use crate::item::Item;

#[async_trait(?Send)]
pub trait ItemContract {
    async fn add(&self, item: Item) -> Result<(), String>;
    async fn list(&self) -> Result<Vec<Item>, String>;
}