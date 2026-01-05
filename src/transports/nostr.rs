use async_trait::async_trait;


pub struct NostrTransport;

#[async_trait]
impl super::Transport for NostrTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via nostr to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via nostr on {}", _address)
    }
}
