use crate::pineapple::{AP, Station};

use libwifi::error::Error;
use libwifi::frame::components::{MacAddress, RsnAkmSuite, WpsSetupState};
use libwifi::frame::{AssociationRequest, AssociationResponse, Beacon, ProbeResponse, QosData};
use libwifi::{Addresses, Frame};

pub fn parse_frame(frame: Frame) -> Result<(Option<AP>, Option<Station>), Error> {
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

fn parse_beacon(beacon_frame: Beacon) -> Result<AP, Error> {
    let station_info = &beacon_frame.station_info;

    let bssid = if let (_, _, Some(bssid)) = parse_header(&beacon_frame.header) {
        bssid
    } else {
        let header = &beacon_frame.header;
        println!(
            "Failed to Parse Beacon: {} {} {}",
            header.address_1.to_long_string(),
            header.address_2.to_long_string(),
            header.address_3.to_long_string(),
        );
        return Err(Error::Failure(
            "Failed to Parse Beacon".to_string(),
            beacon_frame.encode(),
        ));
    };

    let mut akm_suites: Option<Vec<RsnAkmSuite>> = None;
    if let Some(rsn_info) = &station_info.rsn_information {
        akm_suites = Some(rsn_info.akm_suites.to_owned())
    }

    let mut wps_enabled = false;
    if let Some(wps_information) = &station_info.wps_info
        && wps_information.setup_state == WpsSetupState::Configured
    {
        wps_enabled = true;
    }

    Ok(AP::new(
        bssid,
        station_info.essid(),
        akm_suites,
        wps_enabled,
        station_info.channel(),
    ))
}

fn parse_probe_request(probe_response: ProbeResponse) {
    println!("Probe Request:");
    if let (Some(src), dst, _) = parse_header(&probe_response.header) {
        println!("Client {src}->{dst}");
    }
    // parse_station_info(&probe_response.station_info);
}

fn parse_association_request(association_request: AssociationRequest) {
    println!("Association Request:");
    let (src, dst, bssid) = parse_header(&association_request.header);
    // parse_station_info(&association_request.station_info);
    println!(
        "{:<15} -> {:<15} ({:<15})",
        src.unwrap().to_long_string(),
        dst.to_long_string(),
        bssid.unwrap().to_long_string()
    );
}

fn parse_association_response(association_response: AssociationResponse) {
    println!("Association Response:");
    let (src, dst, bssid) = parse_header(&association_response.header);
    // parse_station_info(&association_response.station_info);
    println!(
        "{:<15} -> {:<15} ({:<15})",
        src.unwrap().to_long_string(),
        dst.to_long_string(),
        bssid.unwrap().to_long_string()
    );
}

fn parse_qos_data(qos_data: QosData) {
    println!("QosData:");
    let (src, dst, bssid) = parse_header(&qos_data.header);
    println!(
        "{:<15} -> {:<15} ({:<15})",
        src.unwrap().to_long_string(),
        dst.to_long_string(),
        bssid.unwrap().to_long_string()
    );
    if let Some(eapol) = qos_data.eapol_key {
        // println!("{:?}", &eapol.parse_key_information());
        println!("{:?}", &eapol.determine_key_type());

        // println!("IV   : {:?}", to_hex(&eapol.key_iv));
        // println!("MIC  : {:?}", to_hex(&eapol.key_mic));
        match eapol.determine_key_type() {
            libwifi::frame::MessageType::Message1 => {
                println!("ANonce: {:?}", to_hex(&eapol.key_nonce));
            }
            libwifi::frame::MessageType::Message2 => {
                println!("SNonce: {:?}", to_hex(&eapol.key_nonce));
            }
            libwifi::frame::MessageType::Message3 => {
                println!("ANonce : {:?}", to_hex(&eapol.key_nonce));
                println!("GTK : {:?}", to_hex(&eapol.key_data));
            }
            libwifi::frame::MessageType::Message4 => {}
            libwifi::frame::MessageType::GTK => {}
            libwifi::frame::MessageType::Error => {}
        }
    }
}

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
