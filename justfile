default:
  @echo "You are in {{invocation_directory()}}"
  @just --list

system-info:
  @echo "{{os_family()}} ({{os()}} on {{arch()}} with {{num_cpus()}} cores)"

blinky:
  cd blinky; cargo run

spi:
  cd spi; cargo run

build:
  cd dac; cargo build

test:
  cd mcu-lib; cargo test --all-targets

run:
  cd dac; cargo run
