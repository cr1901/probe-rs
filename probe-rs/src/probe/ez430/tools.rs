use crate::probe::ez430::Ez430Factory;
use crate::probe::DebugProbeInfo;

use std::fmt::Write;


pub(super) fn is_ez430_device(device: &nusb::DeviceInfo) -> bool {
    // Check the VID/PID.
    (device.vendor_id() == 0x451) && (device.product_id() == 0xf432)
}

#[tracing::instrument(skip_all)]
pub(super) fn list_ez430_devices() -> Vec<DebugProbeInfo> {
    let devices = match nusb::list_devices() {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("listing eZ430 devices failed: {:?}", e);
            return vec![];
        }
    };

    devices
        .filter(is_ez430_device)
        .map(|device| {
            DebugProbeInfo::new(
                "eZ430",
                device.vendor_id(),
                device.product_id(),
                None,
                &Ez430Factory,
                None,
            )
        })
        .collect()
}
