use crate::transports::Transport;
use super::gnunet::GnunetTransport;

#[tokio::test]
async fn test_gnunet_transport_connect() {
    let transport = GnunetTransport;
    let address = "gnunet://example.com";
    let result = transport.connect(address).await;
    assert_eq!(result, format!("Connecting via gnunet to {}", address));
}

#[tokio::test]
async fn test_gnunet_transport_listen() {
    let transport = GnunetTransport;
    let address = "0.0.0.0:8080";
    let result = transport.listen(address).await;
    assert_eq!(result, format!("Listening via gnunet on {}", address));
}