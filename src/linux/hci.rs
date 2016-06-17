extern crate libc;
extern crate nix;
extern crate mio;


use super::ffi::*;

use bluetooth::{BtAddr, BtError, BtDevice};

use self::libc::close;
use std::os::raw::*;
use std::ffi::CStr;
use std::ptr;


#[repr(C, packed)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct InquiryInfo {
    pub bdaddr: BtAddr,
    pub pscan_rep_mode: uint8_t,
    pub pscan_period_mode: uint8_t,
    pub pscan_mode: uint8_t,
    pub dev_class: [uint8_t; 3usize],
    pub clock_offset: uint16_t,
}

impl ::std::default::Default for InquiryInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

const IREQ_CACHE_FLUSH: c_long = 1;

// BlueZ funcitons
#[cfg(target_os = "linux")]
#[link(name="bluetooth")]
extern "C" {
    fn hci_get_route(addr: *mut BtAddr) -> c_int /* device_id */;
    fn hci_open_dev(device_id: c_int) -> c_int /* socket to local bluetooth adapter */;

    /* The inquiry last at most for "1.28 * timout" seconds */
    fn hci_inquiry(device_id: c_int, timeout: c_int, max_rsp: c_int, lap: *const u8, inquiry_info: *mut *mut InquiryInfo, flags: c_long) -> c_int;

    fn hci_read_remote_name(socket: c_int, addr: *const BtAddr, max_len: c_int, name: *mut c_char, timeout_ms: c_int) -> c_int;
}

pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    let device_id = unsafe { hci_get_route(0 as *mut BtAddr) };
    if device_id < 0 { return Err(BtError::Desc("hci_get_route(): no local bluetooth adapter found".to_string())); }

    let local_socket = unsafe { hci_open_dev(device_id) };
    if local_socket < 0 { return Err(BtError::Desc("hci_open_dev(): opening local bluetooth adapter failed".to_string())); }

    let mut inquiry_infos = ::std::vec::from_elem(InquiryInfo::default(), 256);

    let timeout = 1; // 1.28 sec
    let flags = IREQ_CACHE_FLUSH;
    let number_responses = unsafe { hci_inquiry(device_id, timeout, inquiry_infos.len() as c_int,
                                                ptr::null(), &mut (&mut inquiry_infos[0] as *mut InquiryInfo), flags) };
    if number_responses < 0 {
        return Err(BtError::Desc("hci_inquiry(): scanning remote bluetooth devices failed".to_string()));
    }

    inquiry_infos.truncate(number_responses as usize);

    let mut devices = Vec::with_capacity(inquiry_infos.len());
    for inquiry_info in &inquiry_infos {
        let mut cname = [0 as c_char; 256];
        let name = if unsafe { hci_read_remote_name(local_socket, &inquiry_info.bdaddr, cname.len() as c_int, &mut cname[0], 0) } < 0 {
            "[unknown]".to_string()
        } else {
            unsafe { CStr::from_ptr(&cname[0]) }.to_string_lossy().into_owned()
        };

        devices.push(BtDevice {
            name: name,
            addr: inquiry_info.bdaddr,
        })
    }

    unsafe { close(local_socket) };

    Ok(devices)
}
