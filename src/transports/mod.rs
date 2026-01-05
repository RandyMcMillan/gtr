use crate::utils::error::GtrResult;
use async_trait::async_trait;

pub mod default;
pub mod torrent;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&self, address: &str) -> GtrResult<()>;
    async fn listen(&self, address: &str) -> GtrResult<()>;
}
