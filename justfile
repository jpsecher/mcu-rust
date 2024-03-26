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
  cd minigrep; cargo build

test:
  cd minigrep; cargo test --all-targets

run needle haystack:
  cd minigrep; cargo run {{needle}} {{haystack}}
