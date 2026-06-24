use std::collections::HashMap;

use libwifi::frame::{
    EapolKey,
    components::{MacAddress, RsnAkmSuite},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct AP {
    clients: HashMap<MacAddress, Client>,
    essid: String,
    pub bssid: MacAddress,
    akm_suites: Option<Vec<RsnAkmSuite>>,
    is_wps_enabled: bool,
    channel: u8,
}

#[derive(Debug, Clone)]
struct Client {
    ssid: MacAddress,
    deauth_cnt: u8,
    message1: Option<EapolKey>,
    message2: Option<EapolKey>,
    message3: Option<EapolKey>,
}

impl AP {
    pub fn new(essid: String, bssid: MacAddress) -> Self {
        AP {
            clients: HashMap::new(),
            essid: essid,
            bssid: bssid,
            akm_suites: None,
            is_wps_enabled: false,
            channel: 0,
        }
    }
    pub fn add_client(&mut self, client_bssid: &MacAddress) {
        self.clients.entry(*client_bssid).or_insert(Client {
            ssid: *client_bssid,
            deauth_cnt: u8::MIN,
            message1: None,
            message2: None,
            message3: None,
        });
    }
    pub fn add_akm(&mut self, akm_suite: &Vec<RsnAkmSuite>) {
        self.akm_suites = Some(akm_suite.clone());
    }

    pub fn set_wps(&mut self, is_wps_enabled: bool) {
        self.is_wps_enabled = is_wps_enabled
    }
    pub fn set_channel(&mut self, channel: &u8) {
        self.channel = *channel;
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
            self.essid,
            self.channel,
            enc_string
        )
    }
}
