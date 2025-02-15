{
  perSystem =
    {
      config,
      self',
      pkgs,
      ...
    }:
    {
      devShells.default = pkgs.mkShell {
        inputsFrom = [
          self'.devShells.rust
          self'.devShells.aliases
          config.pre-commit.devShell
          config.treefmt.build.devShell
        ];

        packages = with pkgs; [
          cargo-auditable
          cargo-tarpaulin # code coverage
          cargo-udeps # unused deps
          aoc-cli
          bacon
        ];

        env.RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
      };
    };
}
