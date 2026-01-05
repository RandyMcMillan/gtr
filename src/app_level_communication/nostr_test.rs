use super::nostr::NostrAppLevelCommunication;
use crate::app_level_communication::ApplicationLevelCommunication;

#[test]
fn test_nostr_app_level_communication_send() {
    let comm = NostrAppLevelCommunication;
    let message = "hello";
    let result = comm.send_message(message);
    assert_eq!(result, format!("Sending message via nostr: {}", message));
}

#[test]
fn test_nostr_app_level_communication_receive() {
    let comm = NostrAppLevelCommunication;
    let result = comm.receive_message();
    assert_eq!(result, "Receiving message via nostr.".to_string());
}
