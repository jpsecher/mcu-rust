default:
  @echo "You are in {{invocation_directory()}}"
  @just --list

system-info:
  @echo "{{os_family()}} ({{os()}} on {{arch()}} with {{num_cpus()}} cores)"

hello:
  rustc hello-world/main.rs

run:
  cd hello-world; cargo run

