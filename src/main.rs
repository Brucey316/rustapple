use clap::{Args, Parser, ValueHint};
use libwifi::error::Error;
use netlink_wi::{MonitorFlags, interface::InterfaceType};
use pcap::{Activated, Capture};
use std::path::PathBuf;
use tokio::time::{Duration, sleep, timeout};

mod frames;
mod interfaces;
mod networking;

use frames::{strip_packet, update_network};
use interfaces::{Interface, InterfaceError};
use networking::{ap::AP, channel::Channel, network::Network, station::Station};

/// A program to attack and gather EAPOL handshakes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Rustapple {
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
    let args = Rustapple::parse();

    match (args.source.interface, args.source.input) {
        (Some(iface), None) => {
            let interface = interface_setup(iface).await?;

            println!("Searching available channels");
            let interface_channels = interface.get_channels().await?;

            let mut network = Network::new();
            for channel in &interface_channels {
                let channel =
                    Channel::new(channel.frequency, channel.no_ir, channel.radar_detection);
                network.add_channel(&channel);
                println!("{}", channel);
            }

            for channel in network.get_channels() {
                interface.set_channel(channel.frequency).await?;
                let timeout_duration = Duration::from_secs(10);

                println!("\nScanning Freq: {}", channel.frequency);
                let capture = Capture::from_device(interface.name.as_str())?
                    .open()?
                    .setnonblock()?;
                read_packets(capture, Some(timeout_duration), &mut network).await;

                println!("Channel Report:");
                println!("---------------");
                if let Some(aps) = network.get_channel_aps(channel) {
                    for ap in aps {
                        println!("{}", ap);
                    }
                } else {
                    println!("ERR: FAILED TO SCAN ON CHANNEL {:#?}", channel);
                }
            }

            interface.set_mode(InterfaceType::Station).await?;
            println!("Managed Mode is set");

            interface.set_down().await?;
            println!("Device is back down");
        }
        (None, Some(path)) => {
            let mut network = Network::new();

            println!("Reading from pcap file: {}", path.display());
            let capture = pcap::Capture::from_file(path);
            match capture {
                Ok(capture) => {
                    read_packets(capture, None, &mut network).await;
                }
                Err(err) => match err {
                    pcap::Error::PcapError(_) => {
                        println!("Caught pcapng error")
                    }
                    _ => {}
                },
            }
        }
        _ => unreachable!(), // clap group guarantees exactly one is set
    }

    Ok(())
}

async fn interface_setup(iface: String) -> Result<Interface, InterfaceError> {
    println!("Turning interface on");
    let interface = Interface::new(&iface).await?;
    interface.set_up().await?;

    println!("Enabling monitor mode");
    interface.set_mode(InterfaceType::Monitor).await?;
    let flags = vec![MonitorFlags::FcsFail, MonitorFlags::Control];
    interface.set_monitor_flags(flags).await?;

    Ok(interface)
}

async fn read_packets<T: Activated>(
    mut capture: Capture<T>,
    max_timeout: Option<Duration>,
    network: &mut Network,
) -> Result<(), Error> {
    let mut ap: Option<AP>;
    let mut station: Option<Station>;

    if let Some(max_timeout) = max_timeout {
        let _ = timeout(max_timeout, async {
            loop {
                match capture.next_packet() {
                    Ok(packet) => {
                        let packet_data = strip_packet(&packet);

                        if let Ok(frame) = libwifi::parse_frame(packet_data.as_slice(), false) {
                            let (ap, station) = frames::parse_frame(&frame)?;
                        } else {
                            //println!("failed to parse")
                        }
                    }
                    Err(err) => match err {
                        pcap::Error::TimeoutExpired => {}
                        _ => {
                            println!("Failed to read packet");
                            return Ok::<(), Error>(());
                        }
                    },
                }
                sleep(Duration::from_millis(0)).await;
            }
        })
        .await;
    } else {
        while let Ok(packet) = capture.next_packet() {
            let packet_data = strip_packet(&packet);

            if let Ok(frame) = libwifi::parse_frame(packet_data.as_slice(), false) {
                let (ap, station) = frames::parse_frame(&frame)?;
                if ap.is_some() || station.is_some() {
                    let _ = update_network(ap, station, &frame, network);

                    println!("found ap");
                }
            } else {
                //println!("failed to parse")
            }
        }
    }

    Ok(())
}
