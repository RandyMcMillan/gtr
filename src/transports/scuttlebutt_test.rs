use crate::transports::Transport;
use super::scuttlebutt::ScuttlebuttTransport;

#[tokio::test]
async fn test_scuttlebutt_transport_connect() {
    let transport = ScuttlebuttTransport;
    let address = "scuttlebutt://example.com";
    let result = transport.connect(address).await;
    assert_eq!(result, format!("Connecting via scuttlebutt to {}", address));
}

#[tokio::test]
async fn test_scuttlebutt_transport_listen() {
    let transport = ScuttlebuttTransport;
    let address = "0.0.0.0:8080";
    let result = transport.listen(address).await;
    assert_eq!(result, format!("Listening via scuttlebutt on {}", address));
}