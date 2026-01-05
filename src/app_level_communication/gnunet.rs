use super::ApplicationLevelCommunication;

pub struct GnunetAppLevelCommunication;

impl ApplicationLevelCommunication for GnunetAppLevelCommunication {
    fn send_message(&self, message: &str) -> String {
        format!("Sending message via gnunet: {}", message)
    }

    fn receive_message(&self) -> String {
        "Receiving message via gnunet.".to_string()
    }
}
