

use crate::{
    probe::{
        DebugProbe, DebugProbeError, DebugProbeInfo, DebugProbeSelector, Probe, ProbeError,
        ProbeFactory, WireProtocol,
    },
    Error as ProbeRsError, MemoryInterface,
};

mod tools;
mod usb_interface;

use usb_interface::Ez430UsbDevice;

/// A factory for creating [`Ez430`] probes.
#[derive(Debug)]
pub struct Ez430Factory;

impl std::fmt::Display for Ez430Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("eZ430")
    }
}

impl ProbeFactory for Ez430Factory {
    fn open(&self, selector: &DebugProbeSelector) -> Result<Box<dyn DebugProbe>, DebugProbeError> {
        let device = Ez430UsbDevice::new_from_selector(selector)?;
        let mut Ez430 = Ez430 {
        };

        Ez430.init()?;

        Ok(Box::new(Ez430))
    }

    fn list_probes(&self) -> Vec<DebugProbeInfo> {
        tools::list_ez430_devices()
    }
}


/// ST-Link specific errors.
#[derive(thiserror::Error, Debug, docsplay::Display)]
pub enum Ez430Error {
    /// USB error.
    Usb(#[from] std::io::Error),
}

impl ProbeError for Ez430Error {}


#[derive(Debug)]
pub struct Ez430 {
}

impl DebugProbe for Ez430 {
    fn get_name(&self) -> &str {
        todo!()
    }

    fn speed_khz(&self) -> u32 {
        todo!()
    }

    fn set_speed(&mut self, speed_khz: u32) -> Result<u32, DebugProbeError> {
        todo!()
    }

    fn set_scan_chain(&mut self, scan_chain: Vec<probe_rs_target::ScanChainElement>) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn scan_chain(&self) -> Result<&[probe_rs_target::ScanChainElement], DebugProbeError> {
        todo!()
    }

    fn attach(&mut self) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn detach(&mut self) -> Result<(), crate::Error> {
        todo!()
    }

    fn target_reset(&mut self) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn target_reset_assert(&mut self) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn target_reset_deassert(&mut self) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn select_protocol(&mut self, protocol: WireProtocol) -> Result<(), DebugProbeError> {
        todo!()
    }

    fn active_protocol(&self) -> Option<WireProtocol> {
        todo!()
    }

    fn into_probe(self: Box<Self>) -> Box<dyn DebugProbe> {
        todo!()
    }
}


impl Ez430 {
    fn init(&mut self) -> Result<(), Ez430Error> {
        todo!()
    }
}
