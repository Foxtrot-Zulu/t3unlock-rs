//! Low-level rusb helpers and constants (placeholders).
//! Verify interface numbers, endpoints, request types on real hardware.

use rusb::{Direction, Recipient, RequestType};

/// Placeholder vendor & product defaults are in `usb::DeviceSelector::defaults()`.
/// Interface typically 0 for mass storage-like devices, but confirm.
pub const INTERFACE: u8 = 0;

/// Example control request values (TOTALLY PLACEHOLDER).
pub const REQ_UNLOCK: u8 = 0xA1;
pub const REQ_STATUS: u8 = 0xA2;

pub fn vendor_out() -> u8 {
    rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Interface)
}

pub fn vendor_in() -> u8 {
    rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Interface)
}
