use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub mod default;
pub mod gnunet;
pub mod holepunch;
pub mod https_ssl;
pub mod nostr;
pub mod scuttlebutt;
pub mod torrent;

#[cfg(test)]
pub mod gnunet_test;
#[cfg(test)]
pub mod holepunch_test;
#[cfg(test)]
pub mod https_ssl_test;
#[cfg(test)]
pub mod nostr_test;
#[cfg(test)]
pub mod scuttlebutt_test;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&self, address: &str) -> GtrResult<()>;
    async fn listen(&self, address: &str) -> GtrResult<()>;
}
