default:
  @just --list

system-info:
  @echo "{{os_family()}} ({{os()}},{{arch()}}) with {{num_cpus()}} cores"
