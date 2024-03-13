default:
  @echo "You are in {{invocation_directory()}}"
  @just --list

system-info:
  @echo "{{os_family()}} ({{os()}} on {{arch()}} with {{num_cpus()}} cores)"

hello:
  cd hello-world; cargo run

blinky:
  cd blinky; cargo run

spi:
  cd spi; cargo run
