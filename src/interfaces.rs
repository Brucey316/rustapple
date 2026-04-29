use futures::stream::TryStreamExt;
use netlink_wi::{
    AsyncNlSocket, MonitorFlags,
    interface::{InterfaceType, WirelessInterface},
    wiphy::{Frequency, PhysicalDevice},
};
use rtnetlink::{Handle, LinkUnspec, new_connection, packet_route::link::LinkMessage};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum InterfaceError {
    InterfaceNotFound(String),
    RtNetlinkError(String),
}
impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterfaceNotFound(name) => write!(f, "Interface {} not found\n", name),
            Self::RtNetlinkError(error) => write!(f, "{}", error),
        }
    }
}
impl Error for InterfaceError {}

pub struct Interface {
    pub name: String,
    handle: Handle,
    nl_socket: AsyncNlSocket,
}

impl Interface {
    pub async fn new(name: &str) -> Result<Self, InterfaceError> {
        let (connection, handle, _) = new_connection().map_err(|_| {
            InterfaceError::RtNetlinkError("Failure to connect to kernel".to_string())
        })?;
        tokio::spawn(connection);

        let nl_socket = AsyncNlSocket::connect()
            .await
            .map_err(|_| InterfaceError::RtNetlinkError("Failed to iterate links".to_string()))?;

        let this_interface = Interface {
            name: name.to_string(),
            handle: handle,
            nl_socket: nl_socket,
        };

        Ok(this_interface)
    }

    async fn get_ip_link_by_name(&self) -> Result<LinkMessage, InterfaceError> {
        let mut net_link = self
            .handle
            .link()
            .get()
            .match_name(self.name.to_string())
            .execute();

        if let Ok(Some(link)) = net_link.try_next().await {
            return Ok(link);
        }
        Err(InterfaceError::InterfaceNotFound(self.name.to_string()))
    }

    pub async fn set_up(&self) -> Result<(), InterfaceError> {
        let link = self.get_ip_link_by_name().await?;
        let index = link.header.index;

        println!("Index found {}", index);

        self.handle
            .link()
            .set(LinkUnspec::new_with_index(index).up().build())
            .execute()
            .await
            .map_err(|_| InterfaceError::RtNetlinkError("Faild to set interface up".to_string()))
    }

    pub async fn set_down(&self) -> Result<(), InterfaceError> {
        let link = self.get_ip_link_by_name().await?;
        let index = link.header.index;

        self.handle
            .link()
            .set(LinkUnspec::new_with_index(index).down().build())
            .execute()
            .await
            .map_err(|_| InterfaceError::RtNetlinkError("Faild to set interface up".to_string()))
    }

    async fn get_wireless_interface(&self) -> Result<WirelessInterface, InterfaceError> {
        if let Ok(interfaces) = self.nl_socket.list_interfaces().await {
            for interface in interfaces {
                println!("{} == {}", interface.name, self.name);
                if interface.name.eq(&self.name) {
                    return Ok(interface);
                }
            }
        }
        Err(InterfaceError::InterfaceNotFound(
            "Error finding wireless device in iw".to_string(),
        ))
    }
    async fn get_physical_device(&self) -> Result<PhysicalDevice, InterfaceError> {
        if let Ok(interface) = self.get_wireless_interface().await {
            if let Ok(Some(phys_device)) = self
                .nl_socket
                .get_physical_device(interface.wiphy_index)
                .await
            {
                return Ok(phys_device);
            }
        }
        Err(InterfaceError::InterfaceNotFound(
            "Error finding wireless device in iw".to_string(),
        ))
    }

    pub async fn get_channels(&self) -> Option<Vec<Frequency>> {
        if let Ok(phys) = self.get_physical_device().await {
            println!("Got the phys");
            let mut supported_freqs: Vec<Frequency> = Vec::new();
            if let Some(band2) = phys.band_2ghz {
                supported_freqs.extend(band2.frequencies);
            }
            if let Some(band5) = phys.band_5ghz {
                supported_freqs.extend(band5.frequencies);
            }
            if let Some(band6) = phys.band_6ghz {
                supported_freqs.extend(band6.frequencies);
            }
            supported_freqs.retain(|freq| !freq.disabled);

            if !supported_freqs.is_empty() {
                return Some(supported_freqs);
            }
        }
        println!("Failed");
        None
    }

    pub async fn set_mode(&self, mode: InterfaceType) -> Result<(), InterfaceError> {
        let wireless_interface = self.get_wireless_interface().await?;

        match self
            .nl_socket
            .set_interface(wireless_interface.interface_index, mode)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(InterfaceError::RtNetlinkError(
                "Failure to set monitor mode".to_string(),
            )),
        }
    }

    pub async fn set_monitor_flags(&self, flags: Vec<MonitorFlags>) -> Result<(), InterfaceError> {
        let wireless_interface = self.get_wireless_interface().await?;
        match self
            .nl_socket
            .set_monitor_flags(wireless_interface.interface_index, flags)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(InterfaceError::RtNetlinkError(
                "Failure to set monitor mode".to_string(),
            )),
        }
    }
}
