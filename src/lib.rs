//! Interact with Bluetooth devices via RFCOMM channels.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#[macro_use] extern crate enum_primitive;

extern crate mio;
extern crate nix;
extern crate libc;

mod bluetooth;
pub use bluetooth::*;

///////////////////////////////////////
// Linux implementation of functions
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
#[allow(unused_variables)] // TODO: remove warnings
mod windows;

mod platform {

    #[cfg(target_os = "linux")]
    pub use linux::*;

    #[cfg(target_os = "windows")]
    pub use windows::*;
}
