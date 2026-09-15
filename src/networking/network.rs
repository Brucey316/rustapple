use libwifi::frame::components::MacAddress;
use std::collections::HashMap;

use crate::networking::{ap::AP, channel::Channel, station::Station};

#[derive(Debug, Clone)]
pub struct Network {
    channels: Vec<Channel>,
    aps: HashMap<MacAddress, AP>,
    stations: HashMap<MacAddress, Station>,

    channel_aps: HashMap<Channel, Vec<MacAddress>>,
    ap_stations: HashMap<MacAddress, Vec<MacAddress>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            channels: Vec::new(),
            aps: HashMap::new(),
            stations: HashMap::new(),
            channel_aps: HashMap::new(),
            ap_stations: HashMap::new(),
        }
    }

    /*
       SETTERS
    */
    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.push(channel);
    }
    pub fn add_AP(&mut self, ap: AP) {
        self.aps.entry(ap.bssid).or_insert(ap);
    }
    pub fn add_station(&mut self, station: Station) {
        self.stations.entry(station.ssid).or_insert(station);
    }
    pub fn assign_ap_to_channel(&mut self, channel: Channel, ap: AP) {
        self.channel_aps
            .entry(channel)
            .or_insert(Vec::new())
            .push(ap.bssid);
    }
    pub fn assign_client_to_ap(&mut self, ap: AP, station: Station) {
        self.ap_stations
            .entry(ap.bssid)
            .or_insert(Vec::new())
            .push(station.ssid);
    }
    /*
       GETTTERS
    */
    pub fn get_channels(&mut self) -> Vec<Channel> {
        self.channels.clone()
    }
    pub fn get_aps(&mut self) -> Vec<AP> {
        self.aps.values().cloned().collect()
    }
    pub fn get_stations(&mut self) -> Vec<Station> {
        self.stations.values().cloned().collect()
    }
    pub fn get_channel_aps(&mut self, channel: Channel) -> Option<Vec<AP>> {
        self.channel_aps.get(&channel).map_or(None, |bssids| {
            Some(
                bssids
                    .iter()
                    .map(|bssid| {
                        self.aps
                            .get(bssid)
                            .expect("Internal Network Structure Desynced")
                    })
                    .cloned()
                    .collect(),
            )
        })
    }
    pub fn get_ap_stations(&mut self, ap: AP) -> Option<Vec<Station>> {
        self.ap_stations.get(&ap.bssid).map_or(None, |ssids| {
            Some(
                ssids
                    .iter()
                    .map(|ssid| {
                        self.stations
                            .get(ssid)
                            .expect("Internal Network Structure Desynced")
                    })
                    .cloned()
                    .collect(),
            )
        })
    }
}
