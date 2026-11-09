mod beacon;
use beacon::parse_beacon;

use crate::networking::{ap::AP, network::Network, station::Station};
use libwifi::Addresses;
use libwifi::Frame;
use libwifi::error::Error;
use libwifi::frame::components::MacAddress;

pub fn parse_frame(frame: &Frame) -> Result<(Option<AP>, Option<Station>), Error> {
    let mut ap: Option<AP> = None;
    let mut _station: Option<Station> = None;

    match frame {
        // AP Specific Packets
        Frame::Beacon(beacon) => {
            let res = parse_beacon(beacon)?;
            ap = Some(res);
        }
        Frame::ProbeResponse(_probe_response) => {
            // parse_probe_request(probe_response)
        }
        // Station Specific Packets
        Frame::AssociationRequest(_association_request) => {
            // parse_association_request(association_request)
        }
        Frame::AssociationResponse(_association_response) => {
            // parse_association_response(association_response)
        }
        Frame::QosData(_qos_data) => { /*parse_qos_data(qos_data)*/ }
        _ => {
            //println!("{:?}", frame)
        }
    }

    Ok((ap, _station))
}

pub fn update_network(
    ap: Option<AP>,
    station: Option<Station>,
    frame: &Frame,
    network: &mut Network,
) {
}

// fn parse_qos_data(qos_data: QosData) {
//     println!("QosData:");
//     let (src, dst, bssid) = parse_header(&qos_data.header);
//     println!(
//         "{:<15} -> {:<15} ({:<15})",
//         src.unwrap().to_long_string(),
//         dst.to_long_string(),
//         bssid.unwrap().to_long_string()
//     );
//     if let Some(eapol) = qos_data.eapol_key {
//         // println!("{:?}", &eapol.parse_key_information());
//         println!("{:?}", &eapol.determine_key_type());

//         // println!("IV   : {:?}", to_hex(&eapol.key_iv));
//         // println!("MIC  : {:?}", to_hex(&eapol.key_mic));
//         match eapol.determine_key_type() {
//             libwifi::frame::MessageType::Message1 => {
//                 println!("ANonce: {:?}", to_hex(&eapol.key_nonce));
//             }
//             libwifi::frame::MessageType::Message2 => {
//                 println!("SNonce: {:?}", to_hex(&eapol.key_nonce));
//             }
//             libwifi::frame::MessageType::Message3 => {
//                 println!("ANonce : {:?}", to_hex(&eapol.key_nonce));
//                 println!("GTK : {:?}", to_hex(&eapol.key_data));
//             }
//             libwifi::frame::MessageType::Message4 => {}
//             libwifi::frame::MessageType::GTK => {}
//             libwifi::frame::MessageType::Error => {}
//         }
//     }
// }

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

fn parse_header(header: &impl Addresses) -> (Option<MacAddress>, MacAddress, Option<MacAddress>) {
    (
        header.src().copied(),
        header.dest().to_owned(),
        header.bssid().copied(),
    )
}

// fn parse_station_info(station_info: &StationInfo) {
//     let ssid: String;
//     let rsn_info: RsnInformation;
//     let ht_info: HTInformation;

//     if let Some(_ssid) = station_info.essid() {
//         ssid = _ssid
//     } else {
//         return;
//     }

//     if let Some(_rsn_info) = &station_info.rsn_information {
//         rsn_info = _rsn_info.clone()
//     } else {
//         return;
//     }
//     if let Some(_ht_info) = &station_info.ht_information {
//         ht_info = _ht_info.clone()
//     } else {
//         return;
//     }
//     if let Some(_channel) = &station_info.channel(){

//     }

//     let supported_ciphers = rsn_info
//         .akm_suites
//         .iter()
//         .map(|suite| match suite {
//             RsnAkmSuite::PSK => "PSK (WPA2 Personal)",
//             RsnAkmSuite::SAE => "SAE (WPA3 Personal)",
//             RsnAkmSuite::EAP => "EAP (Enterprise)",
//             _ => "Other",
//         })
//         .collect::<Vec<&str>>();

//     print!("{:<20} ", ssid);
//     if supported_ciphers.len() == 0 {
//         print!("{:<14} ", "OPEN");
//     } else {
//         print!("{:<14} ", supported_ciphers.join(","));
//     }

//     println!("{:<3}", ht_info.primary_channel);
// }

pub fn strip_packet(packet: &[u8]) -> Vec<u8> {
    // Strip radiotap header if it exists
    if let Ok((_, subpacket)) = radiotap::Radiotap::parse(&packet) {
        subpacket.to_vec()
    } else {
        packet.to_vec()
    }
}
