use libwifi::error::Error;
use libwifi::frame::Beacon;
use libwifi::frame::components::{RsnAkmSuite, WpsSetupState};

use super::AP;
use super::parse_header;

pub fn parse_beacon(beacon_frame: &Beacon) -> Result<AP, Error> {
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
