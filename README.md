# bluetooth-serial-port

Rust library for interacting with the Bluetooth stack via RFCOMM channels.

This library currently only works Linux.

Cargo.toml:

```toml
[dependencies]
bluetooth-serial-port = "0.2"
```

Example:

```rust
extern crate bluetooth_serial_port;
extern crate mio;
use bluetooth_serial_port::{BtProtocol, BtSocket};
use std::io::{Read, Write};
use mio::{EventLoop, Handler, PollOpt, Token, EventSet};

// warning: you really should do some error handling in real code...

// scan for devices
let devices = bluetooth_serial_port::scan_devices().expect("scan_devices() failed");
if devices.len() == 0 { panic!("No devices found"); }

// "device.name" is name string ot the device
// "device.addr" is the MAC address of the device
let device = &devices[0];
println!("Connecting to `{}` ({})", device.name, device.addr.to_string());

// create and connect the RFCOMM socket
let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
socket.connect_rfcomm(device.addr).unwrap();

// BtSocket implements the `Read` and `Write` traits (they're blocking)
let mut buffer = [0; 10];
let num_bytes_read = socket.read(&mut buffer[..]).expect("Reading bytes failed");
let num_bytes_written = socket.write(&buffer[0..num_bytes_read]).expect("Writing bytes failed");
println!("Read `{}` bytes, wrote `{}` bytes", num_bytes_read, num_bytes_written);

// BtSocket also implements `mio::Evented` for async IO
let mut event_loop = EventLoop::new().unwrap();
event_loop.register(&socket, Token(0), EventSet::readable() | EventSet::writable(),
                    PollOpt::edge() | PollOpt::oneshot()).expect("Registering event failed");

// run event loop with some handler...
event_loop.run(&mut NoopHandler).expect("EventLoop failed");
```
