# razer-rs
Rust library and command line interface for controlling Razer devices on Linux

## About
With this library, you can print info for all connected Razer peripherals,
including connected devices and configure device properties. There is also a
command line interface based on
[`razer-cli`](https:/github.com/LoLei/razer-cli) which uses the library to
display all Razer peripheral properties and allow for command line
configuration. The library uses udev drivers directly so it is necessary to add
your user to `plugdev` group in order to use both the cli or any other program
that uses the `razer-rs` lib.

## Installation

```
git clone https://github.com/sk8ersteve/razer-rs.git
cd razer-rs
cargo install --path .
```

This will install to the bin folder targeted by cargo, which needs to be included
in your environment $PATH variable. This will usually be something like
`/home/user/.cargo/bin`.

You also need to install the openrazer driver. Follow the steps for your
distribution [here](https://openrazer.github.io/#download). The pylib and dbus
daemon are not needed so you may only need to install `openrazer-driver-dkms`
instead of `openrazer-meta` which includes everything.

## Usage

### CLI

Run `razer-rs` for basic information about all connected Razer devices.

### Rust lib

TODO