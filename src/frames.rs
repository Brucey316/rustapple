use libwifi::frame::components::{
    HTInformation, MacAddress, ManagementHeader, RsnAkmSuite, RsnInformation, StationInfo,
};
use libwifi::frame::{AssociationRequest, AssociationResponse, Beacon, ProbeResponse};
use libwifi::{Addresses, Frame};

pub fn parse_frame(frame: Frame) {
    // TODO: Learn how to handle each packet individually
    // Are they really all the same??? Am I focused on directionality?

    match frame {
        // AP Specific Packets
        Frame::Beacon(beacon) => parse_beacon(beacon),
        Frame::ProbeResponse(probe_response) => parse_probe_response(probe_response),
        // Station Specific Packets
        Frame::AssociationRequest(association_request) => {
            parse_association_request(association_request)
        }
        Frame::AssociationResponse(association_response) => {
            parse_association_response(association_response)
        }
        _ => {
            println!("{:?}", frame)
        }
    }
}

fn parse_beacon(beacon_frame: Beacon) {
    println!("Beacon Frame:");
    parse_management_header(beacon_frame.header);
    parse_station_info(beacon_frame.station_info);
}

fn parse_probe_response(probe_response: ProbeResponse) {
    println!("Probe Response:");
    parse_management_header(probe_response.header);
    parse_station_info(probe_response.station_info);
}

fn parse_association_request(association_request: AssociationRequest) {
    println!("Association Request:");
    parse_management_header(association_request.header);
    parse_station_info(association_request.station_info);
}
fn parse_association_response(association_response: AssociationResponse) {
    println!("Association Response:");
    parse_management_header(association_response.header);
    parse_station_info(association_response.station_info);
}

fn parse_management_header(
    management_header: ManagementHeader,
) -> (Option<MacAddress>, MacAddress, Option<MacAddress>) {
    let to_ds = management_header.frame_control.to_ds();
    let fm_ds = management_header.frame_control.from_ds();
    //TODO: Learn the DS system better
    match (to_ds, fm_ds) {
        (true, true) => {
            print!("MESH       ")
        }
        (true, false) => {
            print!("STA -> AP  ")
        }
        (false, true) => {
            print!("AP -> STA  ")
        }
        (false, false) => {
            print!("STA -> STA ")
        }
    }

    //TODO: Play with the addesses and determine a better fit for src vs bssid
    println!(
        "{:<15} -> {:<15} ({:<15})",
        management_header.src().unwrap().to_long_string(),
        management_header.dest().to_long_string(),
        management_header.bssid().unwrap().to_long_string()
    );

    return (
        management_header.src().copied(),
        management_header.dest().to_owned(),
        management_header.bssid().copied(),
    );
}

fn parse_station_info(station_info: StationInfo) {
    let ssid: String;
    let rsn_info: RsnInformation;
    let ht_info: HTInformation;

    if let Some(_ssid) = station_info.essid() {
        ssid = _ssid
    } else {
        return;
    }

    if let Some(_rsn_info) = station_info.rsn_information {
        rsn_info = _rsn_info
    } else {
        return;
    }
    if let Some(_ht_info) = station_info.ht_information {
        ht_info = _ht_info
    } else {
        return;
    }

    let supported_ciphers = rsn_info
        .akm_suites
        .iter()
        .map(|suite| match suite {
            RsnAkmSuite::PSK => "PSK (WPA2 Personal)",
            RsnAkmSuite::SAE => "SAE (WPA3 Personal)",
            RsnAkmSuite::EAP => {
                "EAP
             (Enterprise)"
            }
            _ => "Other",
        })
        .collect::<Vec<&str>>();

    print!("{:<20} ", ssid);
    if supported_ciphers.len() == 0 {
        print!("{:<14} ", "OPEN");
    } else {
        print!("{:<14} ", supported_ciphers.join(","));
    }

    println!("{:<3}", ht_info.primary_channel);
}
