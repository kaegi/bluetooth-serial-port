
use std;
use std::result::Result;
use std::io::{Read, Write};
use std::str;
use mio;

use platform;

/// The bluetooth socket.
///
/// Can be used in a `mio::EventLoop`.
#[derive(Debug)]
pub struct BtSocket(platform::BtSocket);

impl BtSocket {
    /// Create an (still) unconnected socket.
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        Ok(From::from(try!(platform::BtSocket::new(protocol))))
    }

    /// Connect to the RFCOMM service on remote device with address `addr`. Channel will be
    /// determined through SDP protocol.
    ///
    /// This function can block for some seconds.
    pub fn connect(&mut self, addr: BtAddr) -> Result<(), BtError> {
        self.0.connect(addr)
    }
}

impl From<platform::BtSocket> for BtSocket {
    fn from(socket: platform::BtSocket) -> BtSocket {
        BtSocket(socket)
    }
}

impl mio::Evented for BtSocket {
    fn register(&self, poll: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        self.0.register(poll, token, interest, opts)
    }

    fn reregister(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        self.0.reregister(selector, token, interest, opts)
    }

    fn deregister(&self, selector: &mut mio::Selector) -> std::io::Result<()> {
        self.0.deregister(selector)
    }
}

impl Read for BtSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for BtSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

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
    pub fn from_str(s: &str) -> Result<BtAddr, ()> {
        let splits_iter = s.split(':');
        let mut addr = BtAddr::any();
        let mut i = 0;
        for split_str in splits_iter {
            if i == 6 || split_str.len() != 2 { return Err(()); } // only 6 values (0 <= i <= 5) are allowed
            let high = try!((split_str.as_bytes()[0] as char).to_digit(16).ok_or(()));
            let low = try!((split_str.as_bytes()[1] as char).to_digit(16).ok_or(()));
            addr.0[i] = (high * 16 + low) as u8;
            i += 1;
        }
        if i != 6 { return Err(()) }
        Ok(addr)
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::ascii::AsciiExt;

    #[test()]
    fn btaddr_from_string() {
        match BtAddr::from_str("00:00:00:00:00:00") {
            Ok(addr) => assert_eq!(addr, BtAddr([0u8; 6])),
            Err(_) => panic!("")
        }

        let fail_strings = ["addr : String", "00:00:00:00:00", "00:00:00:00:00:00:00", "-00:00:00:00:00:00", "0G:00:00:00:00:00"];
        for &s in &fail_strings {
            match BtAddr::from_str(s) {
                Ok(_) => panic!("Somehow managed to parse \"{}\" as an address?!", s),
                Err(_) => ()
            }
        }
    }

    #[test()]
    fn btaddr_to_string() {
        assert_eq!(BtAddr::any().to_string(), "00:00:00:00:00:00");
        assert_eq!(BtAddr([1, 2, 3, 4, 5, 6]).to_string(), "01:02:03:04:05:06");
    }

    #[test()]
    fn btaddr_roundtrips_to_from_str() {
        let addr = BtAddr([0, 22, 4, 1, 33, 192]);
        let addr_string = "00:ff:ee:ee:dd:12";

        assert_eq!(addr, BtAddr::from_str(&addr.to_string()).unwrap());
        assert!(addr_string.eq_ignore_ascii_case(&BtAddr::from_str(addr_string).unwrap().to_string()));
    }

    #[cfg(not(feature = "test_without_hardware"))]
    #[test()]
    fn creates_rfcomm_socket() {
        BtSocket::new(BtProtocol::RFCOMM).unwrap();
    }

    #[cfg(not(feature = "test_without_hardware"))]
    #[test()]
    fn scans_devices() {
        scan_devices().unwrap();
    }
}
