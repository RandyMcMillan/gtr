use super::https_ssl::HttpsSslTransport;
use super::super::Transport;

#[tokio::test]
async fn test_https_ssl_transport_connect() {
    let transport = HttpsSslTransport;
    let result = transport.connect("https://example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_https_ssl_transport_listen() {
    let transport = HttpsSslTransport;
    let result = transport.listen("0.0.0.0:8080").await;
    assert!(result.is_ok());
}
