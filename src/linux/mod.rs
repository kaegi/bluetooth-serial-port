pub use super::*;

#[allow(non_camel_case_types)]
pub mod ffi;

#[cfg(target_os = "linux")]
pub mod platform {
    pub use super::ffi::*;
}
