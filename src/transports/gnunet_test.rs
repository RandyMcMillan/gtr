use super::gnunet::GnunetTransport;
use super::super::Transport;

#[tokio::test]
async fn test_gnunet_transport_connect() {
    let transport = GnunetTransport;
    let result = transport.connect("gnunet://example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gnunet_transport_listen() {
    let transport = GnunetTransport;
    let result = transport.listen("0.0.0.0:8080").await;
    assert!(result.is_ok());
}
