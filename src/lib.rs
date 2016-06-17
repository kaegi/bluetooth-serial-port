#[macro_use] extern crate enum_primitive;
extern crate mio;
extern crate nix;
extern crate libc;

use std::result::Result;
use std::io::{Read, Write};
use std::str;

///////////////////////////////////////
// Linux implementation of functions
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

pub use linux::scan_devices;

///////////////////////////////////////
// TODO: Windows implementation of functions
// #[cfg(target_os = "windows")]
// mod windows;
// #[cfg(target_os = "windows")]
// pub use windows::platform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtError {
    Unknown,
    Errno(u32, String), // unix error numbers
    Desc(String), // error with description
}

impl std::fmt::Display for BtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BtAddr(pub [u8; 6]);

impl std::fmt::Debug for BtAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl BtAddr {
    pub fn any () -> BtAddr {
        BtAddr ([0, 0, 0, 0, 0, 0])
    }

    pub fn from_str(_: &str) -> Option<BtAddr> {
        unimplemented!(); // TODO: implement BtAddr::from_str
    }

    pub fn to_string(&self) -> String {
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BtDevice {
    pub name: String,
    pub addr: BtAddr,
}

pub enum BtProtocol {
    //L2CAP = BTPROTO_L2CAP,
    //HCI = BTPROTO_HCI,
    //SCO = BTPROTO_SCO,
    RFCOMM,// = BTPROTO_RFCOMM,
    //BNEP = BTPROTO_BNEP,
    //CMTP = BTPROTO_CMTP,
    //HIDP = BTPROTO_HIDP,
    //AVDTP = BTPROTO_AVDTP
}

impl BtDevice {
    pub fn new(name: String, addr: BtAddr) -> BtDevice {
        BtDevice { name: name, addr: addr }
    }

}

pub struct BtSocket {
    io: mio::Io,
}


impl BtSocket {
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        let io = try!(platform::new_mio(protocol));
        Ok(From::from(io))
    }

    pub fn connect(&mut self, addr: BtAddr, rc_channel: u32) -> Result<(), BtError> {
        platform::connect(&mut self.io, addr, rc_channel)
    }

    pub fn connect_rfcomm(&mut self, addr: BtAddr) -> Result<(), BtError> {
        platform::connect(&mut self.io, addr, try!(platform::get_rfcomm_channel(addr)) as u32)
    }
}

impl From<mio::Io> for BtSocket {
    fn from(io : mio::Io) -> BtSocket {
        BtSocket { io : io }
    }
}

impl mio::Evented for BtSocket {
    fn register(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        self.io.register(selector, token, interest, opts)
    }

    fn reregister(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        self.io.reregister(selector, token, interest, opts)
    }

    fn deregister(&self, selector: &mut mio::Selector) -> std::io::Result<()> {
        self.io.deregister(selector)
    }
}

impl Read for BtSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.io.read(buf)
    }
}

impl Write for BtSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
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

    #[cfg(not(feature = "test_without_hardware"))]
    #[test()]
    fn creates_rfcomm_socket() {
        BtSocket::new(BluetoothProtocol::RFCOMM).unwrap();
    }
}
