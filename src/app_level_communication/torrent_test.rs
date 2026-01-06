use super::torrent::TorrentAppLevelCommunication;
use crate::app_level_communication::ApplicationLevelCommunication;

#[test]
fn test_torrent_app_level_communication_send() {
    let comm = TorrentAppLevelCommunication;
    let message = "hello";
    let result = comm.send_message(message);
    assert_eq!(result, format!("Sending message via torrent: {}", message));
}

#[test]
fn test_torrent_app_level_communication_receive() {
    let comm = TorrentAppLevelCommunication;
    let result = comm.receive_message();
    assert_eq!(result, "Receiving message via torrent.".to_string());
}
