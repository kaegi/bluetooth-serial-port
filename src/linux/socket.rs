extern crate libc;
extern crate nix;
extern crate mio;

use bluetooth::*;
use std::mem;
use std::error::Error;
use std::os::unix::io::{AsRawFd, FromRawFd};

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
        nix::Error::Sys(errno) => BtError::Errno(errno as u32, error.description().to_string()),
        nix::Error::InvalidPath => BtError::Unknown,
    }
}

pub fn new_mio(proto: BtProtocol) -> Result<mio::Io, BtError> {
    match proto {
        BtProtocol::RFCOMM => {
            let fd = unsafe { libc::socket(AF_BLUETOOTH, libc::SOCK_STREAM, BtProtocolBlueZ::RFCOMM as i32) };
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
