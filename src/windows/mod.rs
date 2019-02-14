use bluetooth::{BtAddr, BtAsync, BtDevice, BtError, BtProtocol};
use mio;
use mio::{Poll, Ready};
use std;
use std::io::{Read, Write};

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct BtSocket {}

impl BtSocket {
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        unimplemented!();
    }
    pub fn connect(&mut self, addr: BtAddr) -> BtSocketConnect {
        unimplemented!();
    }
}

impl mio::Evented for BtSocket {
    fn register(
        &self,
        poll: &Poll,
        token: mio::Token,
        interest: Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        unimplemented!();
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: mio::Token,
        interest: Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, poll: &Poll) -> std::io::Result<()> {
        unimplemented!();
    }
}

impl Read for BtSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}

impl Write for BtSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct BtSocketConnect<'a> {
    addr: BtAddr,
    socket: &'a mut BtSocket,
}
impl<'a> BtSocketConnect<'a> {
    pub fn new(socket: &'a mut BtSocket, addr: BtAddr) -> Self {
        BtSocketConnect {
            addr: addr.clone(),
            socket: socket,
        }
    }

    pub fn advance(&mut self) -> Result<BtAsync, BtError> {
        unimplemented!();
    }
}

impl<'a> mio::Evented for BtSocketConnect<'a> {
    fn register(
        &self,
        poll: &Poll,
        token: mio::Token,
        interest: Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        unimplemented!();
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: mio::Token,
        interest: Ready,
        opts: mio::PollOpt,
    ) -> std::io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, poll: &Poll) -> std::io::Result<()> {
        unimplemented!();
    }
}

pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    unimplemented!()
}
