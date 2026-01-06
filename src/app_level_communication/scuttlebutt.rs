use super::ApplicationLevelCommunication;

pub struct ScuttlebuttAppLevelCommunication;

impl ApplicationLevelCommunication for ScuttlebuttAppLevelCommunication {
    fn send_message(&self, message: &str) -> String {
        format!("Sending message via scuttlebutt: {}", message)
    }

    fn receive_message(&self) -> String {
        "Receiving message via scuttlebutt.".to_string()
    }
}
