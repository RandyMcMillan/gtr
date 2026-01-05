use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct HolepunchTransport;

#[async_trait]
impl super::Transport for HolepunchTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via holepunch to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via holepunch on {}", _address)
    }
}
