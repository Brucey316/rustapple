use libwifi::frame::components::{MacAddress, RsnAkmSuite};
use std::fmt;

#[derive(Debug, Clone)]
pub struct AP {
    pub bssid: MacAddress,
    essid: Option<String>,
    akm_suites: Option<Vec<RsnAkmSuite>>,
    is_wps_enabled: bool,
    channel: Option<u8>,
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

        return write!(
            f,
            "{}  {:<20}  {:<3}  {}",
            self.bssid.to_long_string(),
            self.essid.clone().unwrap_or("HIDDEN".to_string()),
            self.channel.unwrap_or(u8::MAX),
            enc_string
        );
    }
}
