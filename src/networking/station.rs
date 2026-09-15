use libwifi::frame::EapolKey;
use libwifi::frame::components::MacAddress;

#[derive(Debug, Clone)]
pub struct Station {
    pub ssid: MacAddress,
    deauth_cnt: u8,
    message2: Option<EapolKey>,
    message3: Option<EapolKey>,
    message1: Option<EapolKey>,
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
