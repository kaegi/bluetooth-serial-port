use bluetooth::{BtAddr, BtError, BtDevice, BtProtocol};
use mio;
use std;
use std::io::{Read, Write};

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct BtSocket {

}

impl BtSocket {
    pub fn new(protocol: BtProtocol) -> Result<BtSocket, BtError> {
        unimplemented!();
    }
    pub fn connect(&mut self, addr: BtAddr) -> Result<(), BtError> {
        unimplemented!();
    }
}

impl mio::Evented for BtSocket {
    fn register(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        unimplemented!();
    }

    fn reregister(&self, selector: &mut mio::Selector, token: mio::Token, interest: mio::EventSet, opts: mio::PollOpt) -> std::io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, selector: &mut mio::Selector) -> std::io::Result<()> {
        unimplemented!();
    }
}

impl Read for BtSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { unimplemented!() }
}

impl Write for BtSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { unimplemented!() }

    fn flush(&mut self) -> std::io::Result<()> { unimplemented!() }
}

pub fn scan_devices() -> Result<Vec<BtDevice>, BtError> {
    unimplemented!()
}
