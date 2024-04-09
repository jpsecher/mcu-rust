# Programming microcontrollers with Rust

## Plan

- [x] Set up a Rust environment for embedded development.
- [x] Run Blinky on RaspberryPi Pico.
- [x] Control a SPI device from the Pico.

## Goals

- Evaluate how the [Rust ecosystem](https://www.rust-lang.org) compares against traditional embedded development tools.
- Evaluate [Just](https://github.com/casey/just) against tradtional Make.
- Evaluate [DevEnv](https://devenv.sh) as depencency management.

## One-time setup

1. Make sure that you have `libudev-dev`, eg. `sudo apt install libudev-dev`.
1. ~~Install [DevEnv](https://devenv.sh/getting-started/) so that you have a Nix package management infrastructure.~~  See notes.

## Daily use

~~Run `devenv shell` to switch into the development environment where all the needed tools and dependencies are installed, sort of like with Python virtual environments.~~

Run `just` to see the available project-specific commands.

~~Run `exit` to exit the development environment.~~

## Notes

Have not found out how to actually make [devenv control to the rust eco system](https://github.com/cachix/devenv/pull/900), so not using the devenv setup for rust.  Had to do this instead:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    sudo apt install libudev-dev
    cargo install elf2uf2-rs --locked

On NixOS, do `nix-shell -p pkg-config udev` instead of installing libudev, and then run cargo install flip-link & elf2uf2-rs --locked.

Finally do

    rustup target add thumbv6m-none-eabi

For Just:

    wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
    echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
    sudo apt update
    sudo apt install just

Packages

    sudo apt install gdb-arm-none-eabi openocd

## Reading list

- https://github.com/rust-embedded/awesome-embedded-rust
- https://github.com/rudihorn/light-cli
- https://crates.io/crates/nom
- https://docs.rs/crate/cortex-m-quickstart/latest
- https://github.com/knurling-rs
