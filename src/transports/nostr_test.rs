use crate::transports::Transport;
use super::nostr::NostrTransport;

#[tokio::test]
async fn test_nostr_transport_connect() {
    let transport = NostrTransport;
    let address = "nostr://example.com";
    let result = transport.connect(address).await;
    assert_eq!(result, format!("Connecting via nostr to {}", address));
}

#[tokio::test]
async fn test_nostr_transport_listen() {
    let transport = NostrTransport;
    let address = "0.0.0.0:8080";
    let result = transport.listen(address).await;
    assert_eq!(result, format!("Listening via nostr on {}", address));
}