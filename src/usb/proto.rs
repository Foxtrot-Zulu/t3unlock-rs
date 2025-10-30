//! Protocol building blocks (currently illustrative).
//!
//! Replace the placeholder request IDs and frame layouts with those observed
//! in the reference implementation or reverse-engineered from the device.

use crate::errors::UsbError;
use rusb::{DeviceHandle, GlobalContext};
use std::time::Duration;

pub use super::lowlevel::INTERFACE;

/// Build a password-carrying frame (placeholder layout).
pub fn build_password_frame(password: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + password.len());
    frame.push(0x01); // version/verb placeholder
    frame.push(password.len() as u8);
    frame.extend_from_slice(password);
    frame
}

/// Send the unlock frame via a vendor-specific control transfer.
pub fn send_unlock_frame(handle: &mut DeviceHandle<GlobalContext>, frame: &[u8], timeout: Duration) -> anyhow::Result<()> {
    use super::lowlevel::{vendor_out, REQ_UNLOCK};
    let request_type = vendor_out();
    let value = 0;
    let index = INTERFACE as u16;
    let n = handle.write_control(request_type, REQ_UNLOCK, value, index, frame, timeout)?;
    if n != frame.len() {
        return Err(anyhow::anyhow!(UsbError::Io).context("short write on unlock frame"));
    }
    Ok(())
}

/// Query whether the device is locked (placeholder protocol).
pub fn query_locked(handle: &mut DeviceHandle<GlobalContext>, timeout: Duration) -> anyhow::Result<bool> {
    use super::lowlevel::{vendor_in, REQ_STATUS};
    let request_type = vendor_in();
    let value = 0;
    let index = INTERFACE as u16;
    let mut buf = [0u8; 64];
    let n = handle.read_control(request_type, REQ_STATUS, value, index, &mut buf, timeout)?;
    if n == 0 {
        return Err(anyhow::anyhow!(UsbError::Protocol).context("empty status response"));
    }
    // Example: first byte non-zero => locked
    Ok(buf[0] != 0)
}
