# bluetooth-serial-port

Rust library for interacting with the Bluetooth stack via RFCOMM channels.

This library currently only works on Linux/BlueZ. You can find it on
[crates.io](https://crates.io/crates/bluetooth-serial-port).

Cargo.toml:

```toml
[dependencies]
bluetooth-serial-port = "0.3"
```

Important functions:

```rust
bluetooth_serial_port::scan_devices()
BtSocket::new()
BtSocket::connect()
BtSocket::read()
BtSocket::write()
```

Full example:

```rust
extern crate bluetooth_serial_port;
extern crate mio;
use bluetooth_serial_port::{BtProtocol, BtSocket};
use std::io::{Read, Write};
use mio::{EventLoop, Handler, PollOpt, Token, EventSet};

// warning: you really should do some error handling in real code...

fn main() {
    // scan for devices
    let devices = bluetooth_serial_port::scan_devices().unwrap();
    if devices.len() == 0 { panic!("No devices found"); }

    // "device.name" is name string ot the device
    // "device.addr" is the MAC address of the device
    let device = &devices[0];
    println!("Connecting to `{}` ({})", device.name, device.addr.to_string());

    // create and connect the RFCOMM socket
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
    socket.connect(device.addr).unwrap();

    // BtSocket implements the `Read` and `Write` traits (they're blocking)
    let mut buffer = [0; 10];
    let num_bytes_read = socket.read(&mut buffer[..]).unwrap();
    let num_bytes_written = socket.write(&buffer[0..num_bytes_read]).unwrap();
    println!("Read `{}` bytes, wrote `{}` bytes", num_bytes_read, num_bytes_written);

    // BtSocket also implements `mio::Evented` for async IO
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&socket, Token(0), EventSet::readable() | EventSet::writable(),
    PollOpt::edge() | PollOpt::oneshot()).unwrap();

    // run event loop with some handler...
    struct NoopHandler;
    impl Handler for NoopHandler { type Timeout = (); type Message = (); }
    event_loop.run(&mut NoopHandler).unwrap();
}
```
