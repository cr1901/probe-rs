use nusb::DeviceInfo;
use std::{sync::LazyLock, time::Duration};

use crate::probe::{stlink::StlinkError, usb_util::InterfaceExt};

use std::collections::HashMap;

use scroll::{Pread, Pwrite, LE};

use crate::probe::{DebugProbeSelector, ProbeCreationError};

use super::tools::is_ez430_device;
use super::Ez430Error;
use super::Cmd;

const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

pub(crate) struct Ez430UsbDevice {
    handle: hidapi::HidDevice,
}

impl std::fmt::Debug for Ez430UsbDevice {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ez430UsbDevice")
            .field("handle", &"HidDevice")
            .finish()
    }
}

trait Ez430Device {
    fn do_cmd(&mut self, cmd: Cmd, params: Option<&[u32]>, extra: Option<&[u8]>, reply: &mut [u8]) -> Result<(), Ez430Error>;
}

impl Ez430UsbDevice {
    const BUFSIZ: usize = 64;

    /// Creates and initializes a new USB device.
    pub fn new_from_selector(selector: &DebugProbeSelector) -> Result<Self, ProbeCreationError> {
        let device = nusb::list_devices()
            .map_err(ProbeCreationError::Usb)?
            .filter(is_ez430_device)
            .find(|device| selector.matches(device))
            .ok_or(ProbeCreationError::NotFound)?;

        let api = hidapi::HidApi::new().map_err(ProbeCreationError::HidApi)?;
        let handle = api
            .open(device.vendor_id(), device.product_id())
            .map_err(|_| ProbeCreationError::NotFound)?;

        let usb_ez430 = Self {
            handle,
        };

        tracing::debug!("Succesfully attached to Ez430.");

        Ok(usb_ez430)
    }

    // This is a separate function to faciliate sans-io testing.
    fn mk_cmd_buf(unescaped_buf: &mut [u8], escaped_buf: &mut [u8], cmd: Cmd, params: Option<&[u32]>, extra: Option<&[u8]>) -> Result<usize, Ez430Error> {
        let mut ptr = 0;
        ptr += unescaped_buf.pwrite_with(cmd as u8, 0, LE)?;
        
        let cmd_args_type: u8 = match (params, extra) {
            (None, None) => 1,
            (Some(_), None) => 2,
            (None, Some(_)) => 3,
            (Some(_), Some(_)) => 4
        };
        ptr += unescaped_buf.pwrite_with(cmd_args_type, 1, LE)?;

        if let Some(params) = params {
            ptr += unescaped_buf.pwrite_with(params.len() as u16, 2, LE)?;
            
            for (i, p) in (4..).step_by(4).zip(params) {
                ptr += unescaped_buf.pwrite_with(*p, i, LE)?;
            }
        }

        if let Some(e) = extra {
            ptr += unescaped_buf.pwrite_with(e.len() as u32, ptr, LE)?;
            ptr += unescaped_buf.pwrite_with(e, ptr, ())?;
        }

        let crc = X25.checksum(&unescaped_buf[..ptr]);
        ptr += unescaped_buf.pwrite_with(crc, ptr, LE)?;

        let mut eptr = 1;
        eptr += escaped_buf.pwrite_with(0x7Eu8, eptr, LE)?;  // Start of command.

        for b in &unescaped_buf[..ptr] {
            match b {
                0x7d => {
                    eptr += escaped_buf.pwrite_with([0x7Du8, 0x5D].as_ref(), eptr, ())?;
                },
                0x7e => {
                    eptr += escaped_buf.pwrite_with([0x7Du8, 0x5E].as_ref(), eptr, ())?;
                }
                b => {
                    eptr += escaped_buf.pwrite_with(*b, eptr, LE)?;
                }
            }
        }

        eptr += escaped_buf.pwrite_with(0x7Eu8, eptr, LE)?;  // End of command.
        escaped_buf[0] = (eptr - 1) as u8; // -1 because eZ430 requires length minus one.


        Ok(eptr)
    }

    pub fn do_cmd(&mut self, cmd: Cmd, params: Option<&[u32]>, extra: Option<&[u8]>, reply: &mut [u8]) -> Result<(), Ez430Error> {
        let mut unescaped_buf = [0; 64];
        let mut escaped_buf = [0; 65];

        Self::mk_cmd_buf(&mut unescaped_buf, &mut escaped_buf[1..], cmd, params, extra)?;
        escaped_buf[0] = 0; // HIDAPI requires first byte to be Report ID. Is 0 here.
        self.handle.write(&escaped_buf)?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn mk_bufs() -> (Box<[u8]>, Box<[u8]>) {
        (Box::new([0; 64]), Box::new([0; 65]))
    }

    #[test]
    fn test_init_cmd() {
        let (mut u, mut e) = mk_bufs();
        let len = Ez430UsbDevice::mk_cmd_buf(&mut u, &mut e, Cmd::Init, None, None).unwrap();

        assert_eq!([0x06, 0x7E, 0x01, 0x01, 0x16, 0x07, 0x7E].as_ref(), &e[..len]);
        // Test that the init function detects old, unsupported firmware.
    }
}
