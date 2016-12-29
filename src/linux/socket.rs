extern crate libc;
extern crate nix;
extern crate mio;

use bluetooth::{BtError, BtAddr, BtProtocol};
use platform::get_rfcomm_channel;
use std;
use std::io::{Read, Write};
use std::mem;
use std::error::Error;
use std::os::unix::io::{RawFd, FromRawFd, AsRawFd};
use mio::{Poll, Ready};
use std::os::unix::net::UnixStream;
use mio::unix::EventedFd;

#[derive(Debug)]
pub struct BtSocket {
    stream: UnixStream,
}


impl BtSocket {
    pub fn new(proto: BtProtocol) -> Result<BtSocket, BtError> {
        match proto {
            BtProtocol::RFCOMM => {
                let fd = unsafe { libc::socket(AF_BLUETOOTH, libc::SOCK_STREAM, BtProtocolBlueZ::RFCOMM as i32) };
                if fd < 0 {
                    return Err(From::from(nix::Error::last()));
                } else {
                    Ok(BtSocket::from(fd))
                }
            }
        }
    }

    pub fn connect(&mut self, addr: BtAddr) -> Result<(), BtError> {
        let addr = addr.convert_host_byteorder();
        
        let full_address : sockaddr_rc = sockaddr_rc {
            rc_family : AF_BLUETOOTH as u16,
            rc_bdaddr : addr,
            rc_channel : try!(get_rfcomm_channel(addr))
        };

        if unsafe { libc::connect(self.stream.as_raw_fd(), mem::transmute(&full_address), mem::size_of::<sockaddr_rc>() as u32) } < 0 {
            Err(From::from(nix::Error::last()))
        } else {
            Ok(())
        }
    }
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

impl From<nix::Error> for BtError {
    fn from(e: nix::Error) -> BtError {
        match e {
            nix::Error::Sys(errno) => BtError::Errno(errno as u32, e.description().to_string()),
            nix::Error::InvalidPath => BtError::Unknown,
        }
    }
}

impl From<RawFd> for BtSocket {
    fn from(rawfd: RawFd) -> BtSocket {
        BtSocket { stream: unsafe { UnixStream::from_raw_fd(rawfd) }, }
    }
}

impl mio::Evented for BtSocket {
    fn register(&self, poll: &Poll, token: mio::Token, interest: Ready, opts: mio::PollOpt) -> std::io::Result<()> {
        EventedFd(&self.stream.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: mio::Token, interest: Ready, opts: mio::PollOpt) -> std::io::Result<()> {
        EventedFd(&self.stream.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> std::io::Result<()> {
        EventedFd(&self.stream.as_raw_fd()).deregister(poll)
    }
}

impl Read for BtSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for BtSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
