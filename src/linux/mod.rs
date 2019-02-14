#[allow(non_camel_case_types)]
#[allow(dead_code)] // some types are currently not used
mod ffi;

mod hci;
mod sdp;
mod socket;

pub use self::{
    hci::scan_devices,
    socket::{BtSocket, BtSocketConnect},
};
