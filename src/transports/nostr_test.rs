use super::nostr::NostrTransport;
use super::super::Transport;

#[tokio::test]
async fn test_nostr_transport_connect() {
    let transport = NostrTransport;
    let result = transport.connect("nostr://example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_nostr_transport_listen() {
    let transport = NostrTransport;
    let result = transport.listen("0.0.0.0:8080").await;
    assert!(result.is_ok());
}
