pub mod torrent;
pub mod holepunch;
pub mod scuttlebutt;
pub mod gnunet;
pub mod nostr;

pub trait ApplicationLevelCommunication {
    fn send_message(&self, message: &str) -> String;
    fn receive_message(&self) -> String;
}
