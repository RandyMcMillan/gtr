use async_trait::async_trait;


pub struct HttpsSslTransport;

#[async_trait]
impl super::Transport for HttpsSslTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via https/ssl to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via https/ssl on {}", _address)
    }
}
