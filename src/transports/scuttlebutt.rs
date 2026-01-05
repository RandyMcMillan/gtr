use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct ScuttlebuttTransport;

#[async_trait]
impl super::Transport for ScuttlebuttTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement scuttlebutt connect logic
        println!("Connecting via scuttlebutt to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement scuttlebutt listen logic
        println!("Listening via scuttlebutt on {}", _address);
        Ok(())
    }
}
