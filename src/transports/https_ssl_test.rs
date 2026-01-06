use crate::transports::Transport;
use super::https_ssl::HttpsSslTransport;

#[tokio::test]
async fn test_https_ssl_transport_connect() {
    let transport = HttpsSslTransport;
    let address = "https://example.com";
    let result = transport.connect(address).await;
    assert_eq!(result, format!("Connecting via https/ssl to {}", address));
}

#[tokio::test]
async fn test_https_ssl_transport_listen() {
    let transport = HttpsSslTransport;
    let address = "0.0.0.0:8080";
    let result = transport.listen(address).await;
    assert_eq!(result, format!("Listening via https/ssl on {}", address));
}