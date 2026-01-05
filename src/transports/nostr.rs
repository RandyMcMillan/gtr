use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct NostrTransport;

#[async_trait]
impl super::Transport for NostrTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement nostr connect logic
        println!("Connecting via nostr to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement nostr listen logic
        println!("Listening via nostr on {}", _address);
        Ok(())
    }
}
