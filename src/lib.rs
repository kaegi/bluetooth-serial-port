extern crate mio;
extern crate nix;
extern crate libc;

mod ffi;

use std::io::{Read, Write, Result};
use std::ffi::CString;
use std::mem;

use std::os::unix::prelude::AsRawFd;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct BtAddr(pub [u8; 6]);

impl BtAddr {
    pub fn any () -> BtAddr {
        BtAddr ([0, 0, 0, 0, 0, 0])
    }

    pub fn from_string(addr : &str) -> Option<BtAddr> {
        let mut parsed_address : BtAddr = Self::any();
        match CString::new(addr) {
            Ok(a) => {
                if unsafe { ffi::str2ba(a.as_ptr(), &mut parsed_address) } >= 0 {
                    Some(parsed_address)
                }
                else {
                    None
                }
            }
            Err(_) => None
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let ffi_buffer = CString::from_vec_unchecked(vec![0u8; 17]);
            ffi::ba2str(&self, ffi_buffer.as_ptr());
            String::from(ffi_buffer.to_str().unwrap())
        }
    }
}

#[repr(C)]
#[derive(Copy, Debug, Clone)]
struct sockaddr_rc {
    rc_family : libc::sa_family_t,
    rc_bdaddr : BtAddr,
    rc_channel : u8
}

pub enum BluetoothProtocol {
    L2CAP = BTPROTO_L2CAP,
    HCI = BTPROTO_HCI,
    SCO = BTPROTO_SCO,
    RFCOMM = BTPROTO_RFCOMM,
    BNEP = BTPROTO_BNEP,
    CMTP = BTPROTO_CMTP,
    HIDP = BTPROTO_HIDP,
    AVDTP = BTPROTO_AVDTP
}

pub struct BluetoothSocket {
    io : mio::Io
}

const AF_BLUETOOTH : i32 = 31;

const BTPROTO_L2CAP : isize = 0;
const BTPROTO_HCI : isize = 1;
const BTPROTO_SCO : isize = 2;
const BTPROTO_RFCOMM : isize = 3;
const BTPROTO_BNEP : isize = 4;
const BTPROTO_CMTP : isize = 5;
const BTPROTO_HIDP : isize = 6;
const BTPROTO_AVDTP : isize = 7;

impl BluetoothSocket {
    pub fn new(proto : BluetoothProtocol) -> nix::Result<BluetoothSocket> {
        let fd = unsafe { libc::socket(AF_BLUETOOTH, libc::SOCK_STREAM, proto as i32) };

        if fd < 0 {
            Err(nix::Error::last())
        } else {
            Ok(From::from(mio::Io::from_raw_fd(fd)))
        }
    }

    pub fn connect(&mut self, addr: &BtAddr) -> nix::Result<()> {
        let full_address : sockaddr_rc = sockaddr_rc { rc_family : AF_BLUETOOTH as u16,
            rc_bdaddr : *addr,
            rc_channel : 0
        };

        if unsafe { libc::connect(self.io.as_raw_fd(), &full_address as *const _ as *const libc::sockaddr, mem::size_of::<sockaddr_rc>() as u32) } < 0 {
            Err(nix::Error::last())
        } else {
            Ok(())
        }
    }
}

impl From<mio::Io> for BluetoothSocket {
    fn from(io : mio::Io) -> BluetoothSocket {
        BluetoothSocket { io : io }
    }
}

impl mio::Evented for BluetoothSocket {
    fn register(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> Result<()> {
        self.io.register(selector, token, interest, opts)
    }

    fn reregister(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> Result<()> {
        self.io.reregister(selector, token, interest, opts)
    }

    fn deregister(&self, selector: &mut mio::Selector) -> Result<()> {
        self.io.deregister(selector)
    }
}

impl Read for BluetoothSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.io.read(buf)
    }
}

impl Write for BluetoothSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ascii::AsciiExt;

    #[test()]
    fn btaddr_from_string() {
        match BtAddr::from_string("addr : String") {
            Some(_) => panic!("Somehow managed to parse \"addr : String\" as an address?!"),
            None => ()
        }

        match BtAddr::from_string("00:00:00:00:00:00") {
            Some(addr) => assert_eq!(addr, BtAddr([0u8; 6])),
            None => panic!("")
        }
    }

    #[test()]
    fn btaddr_to_string() {
        assert_eq!(BtAddr::any().to_string(), "00:00:00:00:00:00");
        assert_eq!(BtAddr([1, 2, 3, 4, 5, 6]).to_string(), "06:05:04:03:02:01");
    }

    #[test()]
    fn btaddr_roundtrips_to_from_string() {
        let addr = BtAddr([0, 22, 4, 1, 33, 192]);
        let addr_string = "00:ff:ee:ee:dd:12";

        assert_eq!(addr, BtAddr::from_string(&addr.to_string()).unwrap());
        assert!(addr_string.eq_ignore_ascii_case(&BtAddr::from_string(addr_string).unwrap().to_string()));
    }

    #[test()]
    fn creates_rfcomm_socket() {
        BluetoothSocket::new(BluetoothProtocol::RFCOMM).unwrap();
    }
}
