winservice
==========

A very small Rust library to easily set up and run a Windows system service.

[Documentation](#platforms-and-documentation)

## Example Usage

`Cargo.toml`:

```toml
[dependencies]
winservice = "0.1.0"
```

`main.rs`:

```rust
#![no_main]
#![feature(link_args)]
#![link_args = "-Wl,--subsystem,windows"]

use std::os::raw::{c_char, c_int, c_void};
use std::sync::mpsc::Receiver;

#[macro_use]
extern crate winservice;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(hInstance : *const c_void, hPrevInstance : *const c_void,
    lpCmdLine : *const c_char, nCmdShow : c_int) -> c_int
{
    Service!("myService", service_main);
    0
}

fn service_main(args : Vec<String>, end : Receiver<()>) -> u32 {
    loop {
        // Do some work
        if let Ok(_) = end.try_recv() { break; } }
 0 }
```

## Documentation
  * [`x86_64-pc-windows-gnu`](https://fischmax.github.io/doc/winservice/)
