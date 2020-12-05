let
  pkgs = import <nixpkgs> { };
in
  pkgs.mkShell {
    pname = "genesis";
    buildInputs = with pkgs; [
      cargo
      clippy
      rustfmt
      cargo-edit
      cargo-udeps
      rust-analyzer
      linuxPackages_latest.perf
    ];
  }
