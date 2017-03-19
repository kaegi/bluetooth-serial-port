extern crate libc;
extern crate nix;
extern crate mio;


use super::ffi::*;
use super::socket::create_error_from_last;

use bluetooth::{BtAddr, BtDevice, BtError};

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
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

const IREQ_CACHE_FLUSH: c_long = 1;

// BlueZ funcitons
#[cfg(target_os = "linux")]
#[link(name="bluetooth")]
extern "C" {
    fn hci_get_route(addr: *mut BtAddr) -> c_int /* device_id */;
    fn hci_open_dev(device_id: c_int) -> c_int /* socket to local bluetooth adapter */;

    // The inquiry last at most for "1.28 * timout" seconds
    fn hci_inquiry(device_id: c_int, timeout: c_int, max_rsp: c_int, lap: *const u8, inquiry_info: *mut *mut InquiryInfo, flags: c_long) -> c_int;

    fn hci_read_remote_name(socket: c_int, addr: *const BtAddr, max_len: c_int, name: *mut c_char, timeout_ms: c_int) -> c_int;
}

pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    let device_id = unsafe { hci_get_route(0 as *mut BtAddr) };
    if device_id < 0 {
        return Err(create_error_from_last("hci_get_route(): No local bluetooth adapter found"));
    }

    let local_socket = unsafe { hci_open_dev(device_id) };
    if local_socket < 0 {
        return Err(create_error_from_last("hci_open_dev(): Opening local bluetooth adapter failed"));
    }

    let mut inquiry_infos = ::std::vec::from_elem(InquiryInfo::default(), 256);

    let timeout = 1; // 1.28 sec
    let flags = IREQ_CACHE_FLUSH;
    let number_responses = unsafe {
        hci_inquiry(device_id,
                    timeout,
                    inquiry_infos.len() as c_int,
                    ptr::null(),
                    &mut ::std::mem::transmute(&mut inquiry_infos[0]),
                    flags)
    };
    if number_responses < 0 {
        return Err(create_error_from_last("hci_inquiry(): Scanning remote bluetooth devices failed"));
    }

    inquiry_infos.truncate(number_responses as usize);

    let mut devices = Vec::with_capacity(inquiry_infos.len());
    for inquiry_info in &inquiry_infos {
        let mut cname = [0; 256];
        let name = if unsafe {
            hci_read_remote_name(local_socket,
                                 &inquiry_info.bdaddr,
                                 cname.len() as c_int,
                                 &mut cname[0],
                                 0)
        } < 0 {
            "[unknown]".to_string()
        } else {
            unsafe { CStr::from_ptr(&cname[0]) }.to_string_lossy().into_owned()
        };

        devices.push(BtDevice {
            name: name,
            addr: inquiry_info.bdaddr.convert_host_byteorder(),
        })
    }

    if unsafe { close(local_socket) } < 0 {
        return Err(create_error_from_last("close()"));
    }

    Ok(devices)
}
