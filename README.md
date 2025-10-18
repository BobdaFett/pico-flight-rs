# Pico Flight Controller

This project is a simple flight controller used in DCS. It uses the Pico W as a base board.

There are two primary goals for this project:
 - Learn embedded Rust development
 - Learn some basic PCB design

It is not intended to be a fully functioning flight controller, but I will add as much functionality
as I can. I'll add pictures of the PCB design and/or finished product (in real life!) once I have them.

# Dependencies

This project makes use of the `embassy-rs` crate. It handles the definition of the HAL for the
Pico W. I will be using the Pico Probe as my primary debugger, however this project also allows use of
the `elf2uf2-rs` command (installed via `cargo binstall elf22uf2-rs`). Either method will allow you
to flash the firmware to the Pico W easily.

# Contributing

Overall, I don't think it makes sense to contribute to this project. However, feel free to make a fork
and use this repository as an example for your own project! I think embedded Rust has a lot of potential.