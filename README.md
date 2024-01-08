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
