use libwifi::frame::components::MacAddress;
use std::collections::HashMap;

use super::NetworkingError;
use super::{ap::AP, channel::Channel, station::Station};

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
    pub fn add_channel(&mut self, channel: &Channel) {
        self.channels.push(*channel);
    }
    pub fn add_ap(&mut self, ap: &AP) {
        self.aps.entry(ap.bssid).or_insert(ap.clone());
    }
    pub fn add_station(&mut self, station: &Station) {
        self.stations.entry(station.ssid).or_insert(station.clone());
    }
    pub fn assign_ap_to_channel(
        &mut self,
        channel: &Channel,
        ap: &AP,
    ) -> Result<(), NetworkingError> {
        if let Some(aps) = self.channel_aps.get_mut(channel) {
            aps.push(ap.bssid);
            return Ok(());
        }
        return Err(NetworkingError::ChannelNotFound(channel.frequency));
    }
    pub fn assign_station_to_ap(
        &mut self,
        ap: &AP,
        station: &Station,
    ) -> Result<(), NetworkingError> {
        if let Some(stations) = self.ap_stations.get_mut(&ap.bssid) {
            stations.push(station.ssid);
            return Ok(());
        }
        return Err(NetworkingError::ApNotFound(ap.bssid));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_channel() {
        let mut network = Network::new();
        let channel = Channel::new(1u32, false, false);
        network.add_channel(&channel);

        assert_eq!(network.channels.len(), 1);
        assert_eq!(network.channels[0], channel);
    }

    #[test]
    fn add_ap() {
        let test_bssid = &MacAddress([255, 255, 255, 255, 255, 255]);
        let mut network = Network::new();
        let ap = AP::new(*test_bssid, Some("TEST_AP".to_string()), None, false, None);
        network.add_ap(&ap);

        assert_eq!(network.aps.len(), 1);
        assert_eq!(network.aps[test_bssid], ap);
    }

    #[test]
    fn add_station() {
        let test_ssid = &MacAddress([255, 255, 255, 255, 255, 255]);
        let mut network = Network::new();
        let station = Station::new(*test_ssid);

        network.add_station(&station);

        assert_eq!(network.stations.len(), 1);
    }

    #[test]
    fn get_channels() {
        let mut network = Network::new();
        for channel_num in 0..=9u32 {
            let channel = Channel::new(channel_num, false, false);
            network.add_channel(&channel);
        }
        assert_eq!(network.channels.len(), 10);

        let channels = network.get_channels();
        for (index, channel) in channels.iter().enumerate() {
            assert_eq!(channel.frequency, index as u32);
        }
    }

    #[test]
    fn get_aps() {
        let mut network = Network::new();
        for ap_id in 0..=9u8 {
            let ap = AP::new(
                MacAddress([255, 255, 255, ap_id, ap_id, ap_id]),
                format!("TEST{}", ap_id).into(),
                None,
                false,
                None,
            );
            network.add_ap(&ap);
        }

        assert_eq!(network.aps.len(), 10);

        let aps = network.get_aps();
        assert_eq!(aps.len(), network.aps.len());
        assert_eq!(aps.len(), 10);

        for ap_id in 0..=9u8 {
            let ap = AP::new(
                MacAddress([255, 255, 255, ap_id, ap_id, ap_id]),
                format!("TEST{}", ap_id).into(),
                None,
                false,
                None,
            );
            assert!(aps.contains(&ap));
        }
    }

    #[test]
    fn get_stations() {
        let mut network = Network::new();
        for stat_ssid in 0..=9u8 {
            let staton = Station::new(MacAddress([255, 255, 255, stat_ssid, stat_ssid, stat_ssid]));
            network.add_station(&staton);
        }

        let stations = network.get_stations();
        assert_eq!(stations.len(), network.stations.len());
        assert_eq!(stations.len(), 10);

        for stat_ssid in 0..=9u8 {
            let station =
                Station::new(MacAddress([255, 255, 255, stat_ssid, stat_ssid, stat_ssid]));
            assert!(stations.contains(&station));
        }
    }
}
