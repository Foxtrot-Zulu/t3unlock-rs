//! USB facade: discovery, status, unlock orchestration.
//!
//! The specific vendor/product IDs and control transfer constants are **placeholders**.
//! Verify with real hardware. You can override VID/PID via CLI flags or env vars:
//! - `T3UNLOCK_VID`, `T3UNLOCK_PID` (hex, e.g. 04e8).

mod lowlevel;
mod proto;

use crate::errors::{Result as UsbResult, UsbError};
use anyhow::Context;
use rusb::{GlobalContext, DeviceHandle, UsbContext};
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct DeviceSelector {
    pub vid: u16,
    pub pid: u16,
}

impl DeviceSelector {
    pub fn defaults() -> Self {
        // Common Samsung Electronics VID is 0x04e8. PID for T3 may vary by firmware/region.
        // These defaults are best-effort; override if needed.
        let vid = parse_hex_env("T3UNLOCK_VID").unwrap_or(0x04e8);
        let pid = parse_hex_env("T3UNLOCK_PID").unwrap_or(0x61f1);
        Self { vid, pid }
    }

    pub fn from_cli(vid: Option<String>, pid: Option<String>) -> Self {
        let mut sel = Self::defaults();
        if let Some(v) = vid.and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok()) {
            sel.vid = v;
        }
        if let Some(p) = pid.and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok()) {
            sel.pid = p;
        }
        sel
    }
}

fn parse_hex_env(key: &str) -> Option<u16> {
    std::env::var(key).ok().and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok())
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub device_label: String,
    pub present: bool,
    pub locked: Option<bool>,
}

pub fn status(sel: &DeviceSelector) -> anyhow::Result<Status> {
    let present = find_device::<GlobalContext>(sel).is_ok();
    // Without vendor protocol, we can't reliably read lock state.
    Ok(Status {
        device_label: format!("VID=0x{vid:04x} PID=0x{pid:04x}", vid = sel.vid, pid = sel.pid),
        present,
        locked: present.then(|| true), // assume locked if present; refined once protocol is known
    })
}

pub fn unlock(sel: &DeviceSelector, password: &[u8], dry_run: bool, timeout_ms: Option<u64>) -> anyhow::Result<()> {
    if dry_run {
        tracing::info!("DRY RUN: would discover device and perform control transfers to unlock.");
        tracing::info!("DRY RUN: password length = {}", password.len());
        return Ok(());
    }

    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(5000));
    let mut handle = find_device::<GlobalContext>(sel)?;

    // Claim interface & perform protocol sequence
    let iface = proto::INTERFACE;
    handle.claim_interface(iface).map_err(map_libusb)?;

    // Example: send password frame via vendor control transfer (placeholder values)
    let frame = proto::build_password_frame(password);
    proto::send_unlock_frame(&mut handle, &frame, timeout)?;

    // Example: read status
    let locked = proto::query_locked(&mut handle, timeout)?;
    if locked {
        return Err(anyhow::anyhow!(UsbError::BadPassword).context("device still reports locked"));
    }
    Ok(())
}

pub fn doctor() -> anyhow::Result<String> {
    let mut lines = vec![];
    lines.push("Doctor checks:");
    lines.push("- Is the device connected? Use: `lsusb | grep -i samsung`.");
    lines.push("- If running without sudo, install udev rule from contrib/udev/99-t3unlock.rules, then:");
    lines.push("  sudo udevadm control --reload-rules && sudo udevadm trigger");
    lines.push("- Ensure current user is in the plugdev group (or distribution equivalent).");
    lines.push("- Try setting env VID/PID if detection fails: T3UNLOCK_VID=04e8 T3UNLOCK_PID=61f1");
    Ok(lines.join("\n"))
}

fn find_device<C: UsbContext>(sel: &DeviceSelector) -> anyhow::Result<DeviceHandle<C>> {
    for device in rusb::devices().map_err(map_libusb)?.iter() {
        let desc = device.device_descriptor().map_err(map_libusb)?;
        if desc.vendor_id() == sel.vid && desc.product_id() == sel.pid {
            let handle = device.open().map_err(map_libusb)?;
            return Ok(handle);
        }
    }
    Err(anyhow::anyhow!(UsbError::NotFound).context("No matching USB device found"))
}

fn map_libusb(e: rusb::Error) -> anyhow::Error {
    use rusb::Error::*;
    let kind = match e {
        NotFound => crate::errors::UsbError::NotFound,
        Access => crate::errors::UsbError::AccessDenied,
        Busy => crate::errors::UsbError::Busy,
        Timeout => crate::errors::UsbError::Timeout,
        Io => crate::errors::UsbError::Io,
        _ => crate::errors::UsbError::Other(e.to_string()),
    };
    anyhow::anyhow!(kind)
}
