use mio;
use std;
use std::io::{Read, Write};
use std::result::Result;
use std::str;

use crate::platform;

/// The bluetooth socket.
///
/// Can be used with `mio::Poll`.
#[derive(Debug)]
pub struct BtSocket(platform::BtSocket);

impl BtSocket {
    /// Create an (still) unconnected socket.
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        Ok(From::from(platform::BtSocket::new(protocol)?))
    }

    /// Connect to the RFCOMM service on remote device with address `addr`. Channel will be
    /// determined through SDP protocol.
    ///
    /// This function can block for some seconds.
    pub fn connect(&mut self, addr: BtAddr) -> Result<(), BtError> {
        // Create temporary `mio` event loop
        let evtloop = mio::Poll::new().unwrap();
        let token = mio::Token(0);
        let mut events = mio::Events::with_capacity(2);

        // Request a socket connection
        let mut connect = self.0.connect(addr);

        loop {
            match connect.advance()? {
                BtAsync::WaitFor(evented, interest) => {
                    let mut event_received = false;
                    while !event_received {
                        // Register this, single, event source
                        evtloop
                            .register(evented, token, interest, mio::PollOpt::oneshot())
                            .unwrap();

                        // Wait for it to transition to the requested state
                        evtloop.poll(&mut events, None).unwrap();

                        for event in events.iter() {
                            if event.token() == token {
                                event_received = true;
                                evtloop.deregister(evented).unwrap();
                            }
                        }
                    }
                }

                BtAsync::Done => {
                    return Ok(());
                }
            }
        }
    }

    /// Connect to the RFCOMM service on remote device with address `addr`. Channel will be
    /// determined through SDP protocol.
    ///
    /// This function will return immediately and can therefor not indicate most kinds of failures.
    /// Once the connection actually has been established or an error has been determined the socket
    /// will become writable however. It is highly recommended to combine this call with the usage
    /// of `mio` (or some higher level event loop) to get proper non-blocking behaviour.
    pub fn connect_async(&mut self, addr: BtAddr) -> BtSocketConnect {
        BtSocketConnect(self.0.connect(addr))
    }
}

impl From<platform::BtSocket> for BtSocket {
    fn from(socket: platform::BtSocket) -> BtSocket {
        BtSocket(socket)
    }
}

impl mio::Evented for BtSocket {
    fn register(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        self.0.register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        self.0.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> std::io::Result<()> {
        self.0.deregister(poll)
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

/// What needs to happen to advance to the next state an asynchronous process
#[allow(missing_debug_implementations)] // `&mio::Evented` doesn't do `Debug`
pub enum BtAsync<'a> {
    /// Caller needs to wait for the given `Evented` object to reach the given `Ready` state
    WaitFor(&'a mio::Evented, mio::Ready),

    /// Asynchronous transaction has completed
    Done,
}

/// Manages the bluetooth connection process when used from an asynchronous client.
#[derive(Debug)]
pub struct BtSocketConnect<'a>(platform::BtSocketConnect<'a>);

impl<'a> BtSocketConnect<'a> {
    /// Advance the connection process to the next state
    ///
    /// Usage: When receiving a new `BtSocketConnect` instance call this function to get the
    /// connection process started, then wait for the condition requested in `BtAsync` to apply
    /// (by polling for it in a `mio.Poll` instance in general). Once the condition is met, invoke
    /// this function again to advance to the next connect step. Repeat this process until you reach
    /// `BtAsync::Done`, then discard this object and enjoy your established connection.
    pub fn advance(&mut self) -> Result<BtAsync, BtError> {
        self.0.advance()
    }
}

/// Finds a vector of Bluetooth devices in range.
///
/// This function blocks for some seconds.
pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    platform::scan_devices()
}

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
        write!(f, "{:}", std::error::Error::description(self))
    }
}

impl std::error::Error for BtError {
    fn description(&self) -> &str {
        match self {
            BtError::Unknown => "Unknown Bluetooth Error",
            BtError::Errno(_, ref message) => message.as_str(),
            BtError::Desc(ref message) => message.as_str(),
        }
    }
}

/// A 6-byte long MAC address.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BtAddr(pub [u8; 6]);

impl std::fmt::Debug for BtAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl BtAddr {
    /// Returns the MAC address `00:00:00:00:00:00`
    pub fn any() -> BtAddr {
        BtAddr([0, 0, 0, 0, 0, 0])
    }

    /// Linux lower-layers actually hold the address in native byte-order
    /// althrough they are always displayed in network byte-order
    #[doc(hidden)]
    #[inline(always)]
    #[cfg(target_endian = "little")]
    pub fn convert_host_byteorder(mut self) -> BtAddr {
        {
            let (value_1, value_2) = (&mut self.0).split_at_mut(3);
            std::mem::swap(&mut value_1[0], &mut value_2[2]);
            std::mem::swap(&mut value_1[1], &mut value_2[1]);
            std::mem::swap(&mut value_1[2], &mut value_2[0]);
        }

        self
    }

    #[doc(hidden)]
    #[inline(always)]
    #[cfg(target_endian = "big")]
    pub fn convert_host_byteorder(self) -> BtAddr {
        // Public address structure contents are always big-endian
        self
    }

    /// Converts a string of the format `XX:XX:XX:XX:XX:XX` to a `BtAddr`.
    pub fn from_str(s: &str) -> Result<BtAddr, ()> {
        let splits_iter = s.split(':');
        let mut addr = BtAddr::any();
        let mut i = 0;
        for split_str in splits_iter {
            if i == 6 || split_str.len() != 2 {
                return Err(());
            } // only 6 values (0 <= i <= 5) are allowed
            let high = (split_str.as_bytes()[0] as char).to_digit(16).ok_or(())?;
            let low = (split_str.as_bytes()[1] as char).to_digit(16).ok_or(())?;
            addr.0[i] = (high * 16 + low) as u8;
            i += 1;
        }
        if i != 6 {
            return Err(());
        }
        Ok(addr)
    }

    /// Converts `BtAddr` to a string of the format `XX:XX:XX:XX:XX:XX`.
    pub fn to_string(self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
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
    // L2CAP = BTPROTO_L2CAP,
    // HCI = BTPROTO_HCI,
    // SCO = BTPROTO_SCO,
    // BNEP = BTPROTO_BNEP,
    // CMTP = BTPROTO_CMTP,
    // HIDP = BTPROTO_HIDP,
    // AVDTP = BTPROTO_AVDTP
    /// Serial RFCOMM connection to a bluetooth device.
    RFCOMM, // = BTPROTO_RFCOMM */
}

impl BtDevice {
    /// Create a new `BtDevice` manually from a name and addr.
    pub fn new(name: String, addr: BtAddr) -> BtDevice {
        BtDevice { name, addr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test()]
    fn btaddr_from_string() {
        match BtAddr::from_str("00:00:00:00:00:00") {
            Ok(addr) => assert_eq!(addr, BtAddr([0u8; 6])),
            Err(_) => panic!(""),
        }

        let fail_strings = [
            "addr : String",
            "00:00:00:00:00",
            "00:00:00:00:00:00:00",
            "-00:00:00:00:00:00",
            "0G:00:00:00:00:00",
        ];
        for &s in &fail_strings {
            match BtAddr::from_str(s) {
                Ok(_) => panic!("Somehow managed to parse \"{}\" as an address?!", s),
                Err(_) => (),
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
        assert!(
            addr_string.eq_ignore_ascii_case(&BtAddr::from_str(addr_string).unwrap().to_string())
        );
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
