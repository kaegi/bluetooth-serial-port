#[macro_use] extern crate enum_primitive;

mod bluetooth;
pub use bluetooth::*;

///////////////////////////////////////
// Linux implementation of functions
#[cfg(target_os = "linux")]
mod linux;

mod platform {

    #[cfg(target_os = "linux")]
    pub use linux::*;
}
