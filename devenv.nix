{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    # components = ["elf2uf2-rs"];
  };

  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [ 
    just
    git
  ];

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo hello from $GREET";

  enterShell = ''
    hello
    just --version
  '';
}
