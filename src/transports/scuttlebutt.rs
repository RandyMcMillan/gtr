use async_trait::async_trait;


pub struct ScuttlebuttTransport;

#[async_trait]
impl super::Transport for ScuttlebuttTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via scuttlebutt to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via scuttlebutt on {}", _address)
    }
}
