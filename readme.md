# Programming microcontrollers with Rust

## Plan

- [ ] Set up a Rust environment for embedded development.
- [ ] Run Blinky on RaspberryPi Pico.
- [ ] Control a SPI device from the Pico.

## Goals

- Evaluate how the [Rust ecosystem](https://www.rust-lang.org) compares against traditional embedded development tools.
- Evaluate [Just](https://github.com/casey/just) against tradtional Make.
- Evaluate [DevEnv](https://devenv.sh) as depencency management.

## One-time setup

1. Install [DevEnv](https://devenv.sh/getting-started/) so that you have a Nix package management infrastructure.

## Daily use

Run `devenv shell` to switch into the development environment where all the needed tools and dependencies are installed, sort of like with Python virtual environments.  

Run `just` to see the available project-specific commands.

Run `exit` to exit the development environment.
