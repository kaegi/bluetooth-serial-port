extern crate mio;
extern crate nix;
extern crate libc;

use std;
use std::result::Result;
use std::io::{Read, Write};
use std::str;

use platform;

/// Finds a vector of Bluetooth devices in range.
///
/// This function blocks for some seconds.
pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    platform::scan_devices()
}

///////////////////////////////////////
// TODO: Windows implementation of functions
// #[cfg(target_os = "windows")]
// mod windows;
// #[cfg(target_os = "windows")]
// pub use windows::platform;

/// Represents an error which occurred in this library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtError {
    /// No specific information is known.
    Unknown,

    /// On Unix platforms: the error code and an explanation for this error code.
    Errno(u32, String),

    /// This error only has a description.
    Desc(String),
}

impl std::fmt::Display for BtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


/// A 6-byte long MAC address.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BtAddr(pub [u8; 6]);

impl std::fmt::Debug for BtAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl BtAddr {
    /// Returns the MAC address `00:00:00:00:00:00`
    pub fn any () -> BtAddr {
        BtAddr ([0, 0, 0, 0, 0, 0])
    }

    /// Converts a string of the format `XX:XX:XX:XX:XX:XX` to a `BtAddr`.
    pub fn from_str(_: &str) -> Option<BtAddr> {
        unimplemented!(); // TODO: implement BtAddr::from_str
    }

    /// Converts `BtAddr` to a string of the format `XX:XX:XX:XX:XX:XX`.
    pub fn to_string(&self) -> String {
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}


/// A device with its a name and address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BtDevice {
    /// The name of the device.
    pub name: String,

    /// The MAC address of the device.
    pub addr: BtAddr,
}

/// The Bluetooth protocol you can use with this libary.
///
/// Will probably be always `RFCOMM`.
#[derive(Clone, Copy, Debug)]
pub enum BtProtocol {
    //L2CAP = BTPROTO_L2CAP,
    //HCI = BTPROTO_HCI,
    //SCO = BTPROTO_SCO,
    /// Serial RFCOMM connection to a bluetooth device.
    RFCOMM,// = BTPROTO_RFCOMM,
    //BNEP = BTPROTO_BNEP,
    //CMTP = BTPROTO_CMTP,
    //HIDP = BTPROTO_HIDP,
    //AVDTP = BTPROTO_AVDTP
}

impl BtDevice {
    /// Create a new `BtDevice` manually from a name and addr.
    pub fn new(name: String, addr: BtAddr) -> BtDevice {
        BtDevice { name: name, addr: addr }
    }

}

/// The bluetooth socket.
///
/// Can be used in a `mio::EventLoop`.
#[derive(Debug)]
pub struct BtSocket {
    io: mio::Io,
}


impl BtSocket {
    /// Create an (still) unconnected socket.
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        let io = try!(platform::new_mio(protocol));
        Ok(From::from(io))
    }

    /// Connect to the RFCOMM service on remote device with address `addr` by specifing a channel.
    ///
    /// This function can block for some seconds.
    pub fn connect(&mut self, addr: BtAddr, rc_channel: u32) -> Result<(), BtError> {
        platform::connect(&mut self.io, addr, rc_channel)
    }

    /// Connect to the RFCOMM service on remote device with address `addr`. Channel will be
    /// determined through SDP protocol.
    ///
    /// This function can block for some seconds.
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
