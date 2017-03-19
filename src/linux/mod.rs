#[allow(non_camel_case_types)]
#[allow(dead_code)] // some types are currently not used
mod ffi;

mod sdp;
mod hci;
mod socket;

pub use self::socket::{BtSocket, BtSocketConnect};
pub use self::hci::scan_devices;
