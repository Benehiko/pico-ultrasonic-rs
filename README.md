A rust program to use the Raspberry Pi Pico. 

The program uses the [`embassy-rs/embassy`](https://github.com/embassy-rs/embassy)
packages for the WIFI drivers and GPIO interface.

How to get started?

You need to install `rust`, rust toolchain for cross compiling.

The Pico has the `thumbv6m-none-eabi` architecture.

```shell
rustup target add thumbv6m-none-eabi
```

For more detailed instructions, please take a look at
[The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html).

To compile the program you need to export some environment variables that
will be baked into your compiled application.

```shell
export RP_WIFI_NETWORK="MySSID"
export RP_WIFI_PASSWORD="1234"
export RP_SERVER_IP=192.168.1.100
export RP_SERVER_PORT=9000
```

After you have installed all the required dependencies on your OS, run

```shell
cargo build --release
# mount the pico (press and hold bootsel while powering on)
cargo run --release --bin pico-ultrasonic-rs
```

---
Diagnosing your Pico in the wild.

When deploying your Pico into the wild, you might not have the relevant equipment to diagnose your Pico,
such as your laptop.

The software uses the Pico's LEDs to show some level of progress.

On boot the pico will immediately attempt to initialize the wifi drivers to access the on-board LEDs.

The pico will then signal two 1 second blinks to show that it has come alive.

Next it will signal 5 x 1 second blinks to show that it will connect to the Wireless AP.

Next it will be silent for 2 seconds, after which it will attempt the wireless connection.

On each connection attempt it will blink once.

After aquiring a DHCP ip address it will have a static LED.

Where there is no LED, it means the device has somehow restarted or in a frozen state.

After successfully connecting, it will
---
Technical Background links

https://www.handsontec.com/dataspecs/HC-SR04-Ultrasonic.pdf

---

Credits

The HC-SR04 implementation is pretty much a rewrite of
https://github.com/marcoradocchia/hc-sr04
with the difference being the underlying library
that does the GPIO communications. 

Unfortunately development cannot be done with the `std` rust
library on embedded systems such as the pico.
