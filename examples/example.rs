use bluetooth_serial_port::{BtProtocol, BtSocket};
use mio::{Poll, PollOpt, Ready, Token};
use std::io::{Read, Write};

fn main() {
    // scan for devices
    let devices = bluetooth_serial_port::scan_devices().unwrap();
    if devices.len() == 0 {
        panic!("No devices found");
    }

    // "device.addr" is the MAC address of the device
    let device = &devices[0];
    println!(
        "Connecting to `{}` ({})",
        device.name,
        device.addr.to_string()
    );

    // create and connect the RFCOMM socket
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
    socket.connect(device.addr).unwrap();

    // BtSocket implements the `Read` and `Write` traits (they're blocking)
    let mut buffer = [0; 10];
    let num_bytes_read = socket.read(&mut buffer[..]).unwrap();
    let num_bytes_written = socket.write(&buffer[0..num_bytes_read]).unwrap();
    println!(
        "Read `{}` bytes, wrote `{}` bytes",
        num_bytes_read, num_bytes_written
    );

    // BtSocket also implements `mio::Evented` for async IO
    let poll = Poll::new().unwrap();
    poll.register(
        &socket,
        Token(0),
        Ready::readable() | Ready::writable(),
        PollOpt::edge() | PollOpt::oneshot(),
    )
    .unwrap();
    // loop { ... poll events and wait for socket to be readable/writable ... }
}
