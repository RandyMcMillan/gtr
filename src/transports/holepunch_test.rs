use super::holepunch::HolepunchTransport;
use crate::transports::Transport;

#[tokio::test]
async fn test_holepunch_transport_connect() {
    let transport = HolepunchTransport;
    let result = transport.connect("holepunch://example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_holepunch_transport_listen() {
    let transport = HolepunchTransport;
    let result = transport.listen("0.0.0.0:8080").await;
    assert!(result.is_ok());
}
