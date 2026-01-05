use crate::utils::error::GtrResult;
use async_trait::async_trait;

pub struct HolepunchTransport;

#[async_trait]
impl super::Transport for HolepunchTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement holepunch connect logic
        println!("Connecting via holepunch to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement holepunch listen logic
        println!("Listening via holepunch on {}", _address);
        Ok(())
    }
}
