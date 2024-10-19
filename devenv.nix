{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  env.GREET = "Advent of Code Rust";

  packages = with pkgs; [
    aoc-cli
    bacon
  ];

  cachix.enable = false;

  enterShell = ''
    echo "$GREET"
  '';

  languages.rust = {
    enable = true;
    mold.enable = true;
    channel = "stable";
  };

  pre-commit.hooks = {
    clippy.enable = true;
    clippy.settings.allFeatures = true;

    treefmt.enable = true;
    treefmt.settings.formatters = [
      pkgs.rustfmt
    ];
  };

  # See full reference at https://devenv.sh/reference/options/
}
