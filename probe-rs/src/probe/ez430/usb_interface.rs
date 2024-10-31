use nusb::DeviceInfo;
use std::{sync::LazyLock, time::Duration};

use crate::probe::{stlink::StlinkError, usb_util::InterfaceExt};

use std::collections::HashMap;

use crate::probe::{DebugProbeSelector, ProbeCreationError};

use super::tools::{is_ez430_device};

pub(crate) struct Ez430UsbDevice {
    device_handle: nusb::Device,
    interface: nusb::Interface,
}

impl std::fmt::Debug for Ez430UsbDevice {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ez430UsbDevice")
            .field("device_handle", &"DeviceHandle<rusb::Context>")
            .finish()
    }
}

impl Ez430UsbDevice {
    /// Creates and initializes a new USB device.
    pub fn new_from_selector(selector: &DebugProbeSelector) -> Result<Self, ProbeCreationError> {
        let device = nusb::list_devices()
            .map_err(ProbeCreationError::Usb)?
            .filter(is_ez430_device)
            .find(|device| selector.matches(device))
            .ok_or(ProbeCreationError::NotFound)?;

        let device_handle = device.open().map_err(ProbeCreationError::Usb)?;
        tracing::debug!("Aquired handle for probe");

        /* let mut endpoint_out = false;
        let mut endpoint_in = false;
        let mut endpoint_swo = false;

        let config = device_handle.configurations().next().unwrap();
        if let Some(interface) = config.interfaces().next() {
            if let Some(descriptor) = interface.alt_settings().next() {
                for endpoint in descriptor.endpoints() {
                    if endpoint.address() == info.ep_out {
                        endpoint_out = true;
                    } else if endpoint.address() == info.ep_in {
                        endpoint_in = true;
                    } else if endpoint.address() == info.ep_swo {
                        endpoint_swo = true;
                    }
                }
            }
        }

        if !endpoint_out {
            return Err(StlinkError::EndpointNotFound.into());
        }
        if !endpoint_in {
            return Err(StlinkError::EndpointNotFound.into());
        }
        if !endpoint_swo {
            return Err(StlinkError::EndpointNotFound.into());
        } */

        let interface = device_handle
            .claim_interface(1)
            .map_err(ProbeCreationError::Usb)?;

        tracing::debug!("Claimed interface 0 of USB device.");

        let usb_stlink = Self {
            device_handle,
            interface,
        };

        tracing::debug!("Succesfully attached to Ez430.");

        Ok(usb_stlink)
    }
}
