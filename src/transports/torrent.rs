use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct TorrentTransport;

#[async_trait]
impl super::Transport for TorrentTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via torrent to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via torrent on {}", _address)
    }
}
