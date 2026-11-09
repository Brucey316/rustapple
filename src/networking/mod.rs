use libwifi::frame::components::MacAddress;
use std::{error::Error, fmt};

pub mod ap;
pub mod channel;
pub mod network;
pub mod station;

#[derive(Debug)]
pub enum NetworkingError {
    ChannelNotFound(u32),
    ApNotFound(MacAddress),
    StationNotFound(MacAddress),
}
impl fmt::Display for NetworkingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ChannelNotFound(channel) => writeln!(f, "Channel {} not found", channel),
            Self::ApNotFound(bssid) => write!(f, "Ap {} not found", bssid),
            Self::StationNotFound(ssid) => write!(f, "Station {} not found", ssid),
        }
    }
}
impl Error for NetworkingError {}
