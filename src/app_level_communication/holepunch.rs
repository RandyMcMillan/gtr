use super::ApplicationLevelCommunication;

pub struct HolepunchAppLevelCommunication;

impl ApplicationLevelCommunication for HolepunchAppLevelCommunication {
    fn send_message(&self, message: &str) -> String {
        // TODO: Implement holepunch send message logic
        format!("Sending message via holepunch: {}", message)
    }

    fn receive_message(&self) -> String {
        // TODO: Implement holepunch receive message logic
        "Receiving message via holepunch.".to_string()
    }
}
