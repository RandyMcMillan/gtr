use super::scuttlebutt::ScuttlebuttAppLevelCommunication;
use super::super::ApplicationLevelCommunication;

#[test]
fn test_scuttlebutt_app_level_communication_send() {
    let comm = ScuttlebuttAppLevelCommunication;
    let message = "hello";
    let result = comm.send_message(message);
    assert_eq!(result, format!("Sending message via scuttlebutt: {}", message));
}

#[test]
fn test_scuttlebutt_app_level_communication_receive() {
    let comm = ScuttlebuttAppLevelCommunication;
    let result = comm.receive_message();
    assert_eq!(result, "Receiving message via scuttlebutt.".to_string());
}
