use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct TorrentTransport;

#[async_trait]
impl super::Transport for TorrentTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement torrent connect logic
        println!("Connecting via torrent to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement torrent listen logic
        println!("Listening via torrent on {}", _address);
        Ok(())
    }
}
