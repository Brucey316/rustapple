use crate::interfaces::Interface;
use crate::pineapple::AP;
use clap::{Args, Parser, ValueHint};
use libwifi::frame::components::MacAddress;
use netlink_wi::{MonitorFlags, interface::InterfaceType};
use pcap::{Activated, Capture};
use std::{collections::HashMap, path::PathBuf};
use tokio::time::{Duration, sleep, timeout};

mod frames;
mod interfaces;
mod pineapple;

/// A program to attack and gather EAPOL handshakes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct MyArgs {
    #[command(flatten)]
    source: InputSource,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct InputSource {
    /// Name of the interface to use for live capture
    #[arg(long)]
    interface: Option<String>,

    /// Path to a packet capture file (.pcap)
    #[arg(long, value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = MyArgs::parse();

    match (args.source.interface, args.source.input) {
        (Some(iface), None) => {
            println!("Turning interface on");
            let interface = Interface::new(&iface).await?;
            interface.set_up().await?;

            println!("Enabling monitor mode");
            interface.set_mode(InterfaceType::Monitor).await?;
            let flags = vec![MonitorFlags::FcsFail, MonitorFlags::Control];
            interface.set_monitor_flags(flags).await?;

            println!("Searching available channels");
            let interface_channels = interface
                .get_channels()
                .await
                .expect("No Supported Channels Found");

            let mut channel_data: HashMap<u32, HashMap<MacAddress, AP>> = HashMap::new();
            let mut freqs = Vec::new();

            for channel in &interface_channels {
                println!(
                    "Freq {} {} {}",
                    channel.frequency,
                    if channel.no_ir { "(IR)" } else { "" },
                    if channel.radar_detection { "RADAR" } else { "" }
                );
                channel_data.insert(channel.frequency, HashMap::new());
                freqs.push(channel.frequency);
            }

            for freq in freqs {
                interface.set_channel(freq).await?;
                let timeout_duration = Duration::from_secs(10);

                println!("\nScanning Freq: {}", freq);
                let capture = Capture::from_device(interface.name.as_str())?
                    .open()?
                    .setnonblock()?;
                read_packets(capture, Some(timeout_duration), &mut channel_data).await;

                println!("Channel Report:");
                println!("---------------");
                if let Some(aps) = channel_data.get(&freq) {
                    for ap in aps.values() {
                        println!("{}", ap);
                    }
                }
            }

            interface.set_mode(InterfaceType::Station).await?;
            println!("Managed Mode is set");

            interface.set_down().await?;
            println!("Device is back down");
        }
        (None, Some(path)) => {
            let mut channel_data: HashMap<u32, HashMap<MacAddress, AP>> = HashMap::new();
            channel_data.insert(0u32, HashMap::new());

            println!("Reading from pcap file: {}", path.display());
            let capture = pcap::Capture::from_file(path)?;
            read_packets(capture, None, &mut channel_data).await;
        }
        _ => unreachable!(), // clap group guarantees exactly one is set
    }

    Ok(())
}

async fn read_packets<T: Activated>(
    mut capture: Capture<T>,
    max_timeout: Option<Duration>,
    aps: &mut HashMap<u32, HashMap<MacAddress, AP>>,
) {
    if let Some(max_timeout) = max_timeout {
        let _ = timeout(max_timeout, async {
            loop {
                match capture.next_packet() {
                    Ok(packet) => {
                        let packet_data: &[u8];
                        // Strip radiotap header if it exists
                        if let Ok((_, subpacket)) = radiotap::Radiotap::parse(&packet) {
                            packet_data = subpacket;
                        } else {
                            packet_data = &packet;
                        }

                        if let Ok(frame) = libwifi::parse_frame(packet_data, false) {
                            frames::parse_frame(frame, aps)
                        } else {
                            //println!("failed to parse")
                        }
                    }
                    Err(err) => match err {
                        pcap::Error::TimeoutExpired => {}
                        _ => {
                            println!("Failed to read packet");
                            return;
                        }
                    },
                }
                sleep(Duration::from_millis(0)).await;
            }
        })
        .await;
    } else {
        while let Ok(packet) = capture.next_packet() {
            let packet_data: &[u8];
            // Strip radiotap header if it exists
            if let Ok((_, subpacket)) = radiotap::Radiotap::parse(&packet) {
                packet_data = subpacket;
            } else {
                packet_data = &packet;
            }

            if let Ok(frame) = libwifi::parse_frame(packet_data, false) {
                frames::parse_frame(frame, aps)
            } else {
                println!("failed to parse")
            }
        }
    }
}
