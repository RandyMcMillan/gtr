use async_trait::async_trait;

use crate::utils::error::GtrResult;

pub struct HttpsSslTransport;

#[async_trait]
impl super::Transport for HttpsSslTransport {
    async fn connect(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement https/ssl connect logic
        println!("Connecting via https/ssl to {}", _address);
        Ok(())
    }

    async fn listen(&self, _address: &str) -> GtrResult<()> {
        // TODO: Implement https/ssl listen logic
        println!("Listening via https/ssl on {}", _address);
        Ok(())
    }
}
