use libwifi::frame::EapolKey;
use libwifi::frame::components::MacAddress;

#[derive(Debug, Clone)]
pub struct Station {
    pub ssid: MacAddress,
    deauth_cnt: u8,
    message1: Option<EapolKey>,
    message2: Option<EapolKey>,
    message3: Option<EapolKey>,
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

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        fn msg_equates(msg: &Option<EapolKey>, other_msg: &Option<EapolKey>) -> bool {
            if msg.is_none() && other_msg.is_none() {
                return true;
            }
            if msg.is_none() != other_msg.is_none() {
                return false;
            }

            msg.as_ref().unwrap().key_nonce == other_msg.as_ref().unwrap().key_nonce
                && msg.as_ref().unwrap().key_data == other_msg.as_ref().unwrap().key_data
                && msg.as_ref().unwrap().key_mic == other_msg.as_ref().unwrap().key_mic
        }

        self.ssid == other.ssid
            && self.deauth_cnt == other.deauth_cnt
            && msg_equates(&self.message1, &other.message1)
            && msg_equates(&self.message2, &other.message2)
            && msg_equates(&self.message3, &other.message3)
    }
}
