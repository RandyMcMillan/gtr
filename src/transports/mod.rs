use crate::utils::error::GtrResult;
use async_trait::async_trait;

pub mod default;
pub mod torrent;
pub mod nostr;
pub mod https_ssl;
pub mod holepunch;
pub mod scuttlebutt;
pub mod gnunet;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&self, address: &str) -> GtrResult<()>;
    async fn listen(&self, address: &str) -> GtrResult<()>;
}
