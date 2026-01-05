use super::ApplicationLevelCommunication;

pub struct HolepunchAppLevelCommunication;

impl ApplicationLevelCommunication for HolepunchAppLevelCommunication {
    fn send_message(&self, message: &str) -> String {
        format!("Sending message via holepunch: {}", message)
    }

    fn receive_message(&self) -> String {
        "Receiving message via holepunch.".to_string()
    }
}
