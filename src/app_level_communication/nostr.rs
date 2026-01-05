use super::ApplicationLevelCommunication;

pub struct NostrAppLevelCommunication;

impl ApplicationLevelCommunication for NostrAppLevelCommunication {
    fn send_message(&self, message: &str) -> String {
        format!("Sending message via nostr: {}", message)
    }

    fn receive_message(&self) -> String {
        "Receiving message via nostr.".to_string()
    }
}
