use crate::app_level_communication::ApplicationLevelCommunication;
use super::holepunch::HolepunchAppLevelCommunication;

#[test]
fn test_holepunch_app_level_communication_send() {
    let comm = HolepunchAppLevelCommunication;
    let message = "hello";
    let result = comm.send_message(message);
    assert_eq!(result, format!("Sending message via holepunch: {}", message));
}

#[test]
fn test_holepunch_app_level_communication_receive() {
    let comm = HolepunchAppLevelCommunication;
    let result = comm.receive_message();
    assert_eq!(result, "Receiving message via holepunch.".to_string());
}