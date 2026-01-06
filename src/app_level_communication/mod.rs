pub mod gnunet;
pub mod holepunch;
pub mod nostr;
pub mod scuttlebutt;
pub mod torrent;

#[cfg(test)]
pub mod gnunet_test;
#[cfg(test)]
pub mod holepunch_test;
#[cfg(test)]
pub mod nostr_test;
#[cfg(test)]
pub mod scuttlebutt_test;
#[cfg(test)]
pub mod torrent_test;

pub trait ApplicationLevelCommunication {
    fn send_message(&self, message: &str) -> String;
    fn receive_message(&self) -> String;
}
