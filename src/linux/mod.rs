#[allow(non_camel_case_types)]
mod ffi;

mod sdp;
mod hci;
mod socket;

pub use self::socket::{BtSocket, BtSocketConnect};
pub use self::hci::scan_devices;
