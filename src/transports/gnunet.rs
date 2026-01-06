use async_trait::async_trait;


pub struct GnunetTransport;

#[async_trait]
impl super::Transport for GnunetTransport {
    async fn connect(&self, _address: &str) -> String {
        format!("Connecting via gnunet to {}", _address)
    }

    async fn listen(&self, _address: &str) -> String {
        format!("Listening via gnunet on {}", _address)
    }
}
