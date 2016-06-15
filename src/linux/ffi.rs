extern crate libc;
extern crate nix;

pub use super::*;
use self::libc::close;
use ::std::os::raw::*;
use ::std::ffi::CStr;
use ::std::borrow::Cow;
use ::std::ptr;

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;


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
    fn hci_inquiry(device_id: c_int, timeout: c_int, max_rsp: c_int, lap: *const u8, inquiry_info: *mut InquiryInfo, flags: c_long) -> c_int;

    fn hci_read_remote_name(socket: c_int, addr: *const BtAddr, max_len: c_int, name: *mut c_char, timeout_ms: c_int) -> c_int;
}

pub fn scan_devices() -> Result<Vec<BtDevice>, Cow<'static, str>> {
    unsafe {
        let device_id = hci_get_route(0 as *mut BtAddr);
        if device_id < 0 { return Err(Cow::Borrowed("hci_get_route(): no local bluetooth adapter found")); }

        let local_socket = hci_open_dev(device_id);
        if local_socket < 0 { return Err(Cow::Borrowed("hci_open_dev(): opening local bluetooth adapter failed")); }

        let mut inquiry_infos = ::std::vec::from_elem(InquiryInfo::default(), 256);

        let timout = 8; // 1.28 sec
        let flags = IREQ_CACHE_FLUSH;
        let number_responses = hci_inquiry(device_id, timout, inquiry_infos.len() as c_int, ptr::null(), &mut inquiry_infos[0], flags);
        if number_responses < 0 {
             return Err(Cow::Borrowed("hci_inquiry(): scanning remote bluetooth devices failed"));
        }

        inquiry_infos.truncate(number_responses as usize);

        let mut devices = Vec::with_capacity(inquiry_infos.len());
        for inquiry_info in &inquiry_infos {
            let mut cname = [0 as c_char; 256];
            let name = if hci_read_remote_name(local_socket, &inquiry_info.bdaddr, cname.len() as c_int, &mut cname[0], 0) < 0 {
                "[unknown]".to_string()
            } else {
                CStr::from_ptr(&cname[0]).to_string_lossy().into_owned()
            };

            devices.push(BtDevice {
                name: name,
                addr: inquiry_info.bdaddr,
            })
        }

        close(local_socket);

        Ok(devices)
    }
}


pub mod native_bt_io {
    extern crate libc;
    extern crate nix;
    extern crate mio;

    pub use super::*;
    use ::std::mem;
    use ::std::error::Error;
    use ::std::borrow::Cow;
    use ::std::os::unix::io::{AsRawFd, FromRawFd};

    const AF_BLUETOOTH : i32 = 31;

    const BTPROTO_L2CAP : isize = 0;
    const BTPROTO_HCI : isize = 1;
    const BTPROTO_SCO : isize = 2;
    const BTPROTO_RFCOMM : isize = 3;
    const BTPROTO_BNEP : isize = 4;
    const BTPROTO_CMTP : isize = 5;
    const BTPROTO_HIDP : isize = 6;
    const BTPROTO_AVDTP : isize = 7;

    #[allow(dead_code)]
    enum BtProtocolBlueZ {
        L2CAP = BTPROTO_L2CAP,
        HCI = BTPROTO_HCI,
        SCO = BTPROTO_SCO,
        RFCOMM = BTPROTO_RFCOMM,
        BNEP = BTPROTO_BNEP,
        CMTP = BTPROTO_CMTP,
        HIDP = BTPROTO_HIDP,
        AVDTP = BTPROTO_AVDTP
    }



    #[repr(C)]
    #[derive(Copy, Debug, Clone)]
    struct sockaddr_rc {
        rc_family : libc::sa_family_t,
        rc_bdaddr : BtAddr,
        rc_channel : u8
    }

    fn nix_error_to_bterror(error: nix::Error) -> BtError {
        match nix::Error::last() {
            nix::Error::Sys(errno) => BtError::Errno(errno as u32, Cow::Owned(error.description().to_string())),
            nix::Error::InvalidPath => BtError::Unknown,
        }
    }

    pub fn new_mio(proto: BtProtocol) -> Result<mio::Io, BtError> {
        match proto {
            BtProtocol::RFCOMM => {
                let fd = unsafe { libc::socket(AF_BLUETOOTH, libc::SOCK_STREAM, proto as i32) };
                if fd < 0 {
                    Err(nix_error_to_bterror(nix::Error::last()))
                } else {
                    Ok(mio::Io::from_raw_fd(fd))
                }
            }
        }
    }

    pub fn connect(io: &mut mio::Io, addr: BtAddr, rc_channel: u32) -> Result<(), BtError> {
        let full_address : sockaddr_rc = sockaddr_rc {
            rc_family : AF_BLUETOOTH as u16,
            rc_bdaddr : addr,
            rc_channel : rc_channel as u8
        };

        if unsafe { libc::connect(io.as_raw_fd(), &full_address as *const _ as *const libc::sockaddr, mem::size_of::<sockaddr_rc>() as u32) } < 0 {
            Err(nix_error_to_bterror(nix::Error::last()))
        } else {
            Ok(())
        }
    }
}
