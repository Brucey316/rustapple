use crate::interfaces::Interface;
use clap::{Args, Parser, ValueHint};
use netlink_wi::{MonitorFlags, interface::InterfaceType};
use pcap::{Activated, Capture};
use std::path::PathBuf;

mod frames;
mod interfaces;

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
            println!("Opening live capture on interface: {}", iface);
            // open_live_capture(&iface);
            let interface = Interface::new(&iface).await?;
            interface.set_up().await?;

            interface.set_mode(InterfaceType::Monitor).await?;
            let flags = vec![MonitorFlags::FcsFail, MonitorFlags::Control];
            interface.set_monitor_flags(flags).await?;
            println!("Monitor Mode is set");

            let capture = Capture::from_device(interface.name.as_str())?.open()?;
            read_packets(capture);

            interface.set_mode(InterfaceType::Station).await?;
            println!("Managed Mode is set");

            interface.set_down().await?;
            println!("Device is back down");
        }
        (None, Some(path)) => {
            println!("Reading from pcap file: {}", path.display());
            let capture = pcap::Capture::from_file(path)?;
            read_packets(capture);
        }
        _ => unreachable!(), // clap group guarantees exactly one is set
    }

    Ok(())
}

fn read_packets<T: Activated>(mut capture: Capture<T>) {
    while let Ok(packet) = capture.next_packet() {
        let (_, subpacket) = radiotap::Radiotap::parse(&packet).unwrap();

        if let Ok(frame) = libwifi::parse_frame(subpacket, true) {
            frames::parse_frame(frame)
        } else {
            println!("failed to parse")
        }
    }
}
