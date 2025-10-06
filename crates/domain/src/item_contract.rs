use async_trait::async_trait;
use crate::item::Item;
use crate::wie_error::WieError;

#[async_trait(?Send)]
pub trait ItemContract {
    async fn create(&self, item: Item) -> Result<Item, WieError>;
    async fn get(&self, id: &str) -> Result<Option<Item>, WieError>;
    async fn update(&self, item: Item) -> Result<Item, WieError>;
    async fn delete(&self, id: &str) -> Result<bool, WieError>;
    async fn list(&self) -> Result<Vec<Item>, WieError>;
}