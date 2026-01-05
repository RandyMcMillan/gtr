use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct GnunetTransport;

#[async_trait]
impl super::Transport for GnunetTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement gnunet connect logic
        println!("Connecting via gnunet to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement gnunet listen logic
        println!("Listening via gnunet on {}", _address);
        Ok(())
    }
}
