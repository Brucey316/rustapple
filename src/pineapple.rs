use std::{collections::HashMap, u8};

use libwifi::frame::EapolKey;
use libwifi::frame::components::{MacAddress, RsnAkmSuite};

use std::fmt;

#[derive(Debug, Clone)]
pub struct Network {
    channels: Vec<Channel>,
    aps: HashMap<MacAddress, AP>,
    stations: HashMap<MacAddress, Station>,

    channel_aps: HashMap<Channel, Vec<MacAddress>>,
    ap_stations: HashMap<MacAddress, Vec<MacAddress>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Channel {
    pub frequency: u32,
    pub ir: bool,
    pub radar_detection: bool,
}

#[derive(Debug, Clone)]
pub struct AP {
    bssid: MacAddress,
    essid: Option<String>,
    akm_suites: Option<Vec<RsnAkmSuite>>,
    is_wps_enabled: bool,
    channel: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Station {
    ssid: MacAddress,
    deauth_cnt: u8,
    message2: Option<EapolKey>,
    message3: Option<EapolKey>,
    message1: Option<EapolKey>,
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

impl Channel {
    pub fn new(frequency: u32, ir: bool, radar_detection: bool) -> Self {
        Channel {
            frequency: frequency,
            ir: ir,
            radar_detection: radar_detection,
        }
    }
}
impl AP {
    pub fn new(
        bssid: MacAddress,
        essid: Option<String>,
        akm_suites: Option<Vec<RsnAkmSuite>>,
        is_wps_enabled: bool,
        channel: Option<u8>,
    ) -> Self {
        AP {
            bssid,
            essid,
            akm_suites,
            is_wps_enabled,
            channel,
        }
    }
}
impl Station {
    pub fn new(ssid: MacAddress) -> Self {
        Station {
            ssid,
            deauth_cnt: 0u8,
            message1: None,
            message2: None,
            message3: None,
        }
    }

    // pub fn add_EAPOL_Key(&mut self, msg: EapolKey) {
    //     match (msg) {
    //         MessageType::Message1 => {
    //             self.message1 = Some(msg);
    //             self.message1.unwrap();
    //         }
    //         MessageType::Message2 => {
    //             self.message2 = Some(msg);
    //         }
    //         MessageType::Message3 => {
    //             self.message3 = Some(msg);
    //         }
    //         _ => {}
    //     }
    // }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Freq {}{}{}",
            self.frequency,
            if self.ir { "(IR)" } else { "" },
            if self.radar_detection { "RADAR" } else { "" }
        )
    }
}

impl fmt::Display for AP {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        let mut enc_string = "None".to_string();
        if let Some(akm_suites) = &self.akm_suites {
            let cipher_suites = akm_suites
                .iter()
                .map(|suite| match suite {
                    RsnAkmSuite::PSK => "PSK (WPA2 Personal)",
                    RsnAkmSuite::SAE => "SAE (WPA3 Personal)",
                    RsnAkmSuite::EAP => "EAP (Enterprise)",
                    _ => "Other",
                })
                .collect::<Vec<&str>>();
            enc_string = cipher_suites.join(",");
        }

        write!(
            f,
            "{}  {:<20}  {:<3}  {}",
            self.bssid.to_long_string(),
            self.essid.clone().unwrap_or("HIDDEN".to_string()),
            self.channel.unwrap_or(u8::MAX),
            enc_string
        )
    }
}
