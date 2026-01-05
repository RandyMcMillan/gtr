use crate::utils::error::GtrResult;
use async_trait::async_trait;

pub mod default;
pub mod torrent;
pub mod nostr;
pub mod https_ssl;
pub mod holepunch;
pub mod scuttlebutt;
pub mod gnunet;

#[cfg(test)]
pub mod https_ssl_test;
#[cfg(test)]
pub mod holepunch_test;
#[cfg(test)]
pub mod scuttlebutt_test;
#[cfg(test)]
pub mod gnunet_test;
#[cfg(test)]
pub mod nostr_test;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&self, address: &str) -> GtrResult<()>;
    async fn listen(&self, address: &str) -> GtrResult<()>;
}
