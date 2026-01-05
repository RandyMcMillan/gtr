use super::gnunet::GnunetAppLevelCommunication;
use super::super::ApplicationLevelCommunication;

#[test]
fn test_gnunet_app_level_communication_send() {
    let comm = GnunetAppLevelCommunication;
    let message = "hello";
    let result = comm.send_message(message);
    assert_eq!(result, format!("Sending message via gnunet: {}", message));
}

#[test]
fn test_gnunet_app_level_communication_receive() {
    let comm = GnunetAppLevelCommunication;
    let result = comm.receive_message();
    assert_eq!(result, "Receiving message via gnunet.".to_string());
}
