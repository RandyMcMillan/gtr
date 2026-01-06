use crate::transports::Transport;
use super::holepunch::HolepunchTransport;

#[tokio::test]
async fn test_holepunch_transport_connect() {
    let transport = HolepunchTransport;
    let address = "holepunch://example.com";
    let result = transport.connect(address).await;
    assert_eq!(result, format!("Connecting via holepunch to {}", address));
}

#[tokio::test]
async fn test_holepunch_transport_listen() {
    let transport = HolepunchTransport;
    let address = "0.0.0.0:8080";
    let result = transport.listen(address).await;
    assert_eq!(result, format!("Listening via holepunch on {}", address));
}