# bluetooth-serial-port

Rust library for interacting with the Bluetooth stack via RFCOMM channels.

This library currently only works on Linux/BlueZ. You can find it on
[crates.io](https://crates.io/crates/bluetooth-serial-port).

Cargo.toml:

```toml
[dependencies]
bluetooth-serial-port = "0.5.0"
```

Important functions:

```rust
bluetooth_serial_port::scan_devices()
BtSocket::new()
BtSocket::connect()
BtSocket::connect_async()
BtSocket::read()
BtSocket::write()

impl mio::Evented for BtSocket { ... } // for async IO with mio
```

[Click here](examples/example.rs) for full example.
