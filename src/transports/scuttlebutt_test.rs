use super::scuttlebutt::ScuttlebuttTransport;
use super::super::Transport;

#[tokio::test]
async fn test_scuttlebutt_transport_connect() {
    let transport = ScuttlebuttTransport;
    let result = transport.connect("scuttlebutt://example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scuttlebutt_transport_listen() {
    let transport = ScuttlebuttTransport;
    let result = transport.listen("0.0.0.0:8080").await;
    assert!(result.is_ok());
}
