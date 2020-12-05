let
  pkgs = import <nixpkgs> { };
in
  pkgs.mkShell {
    pname = "genesis";
    buildInputs = with pkgs; [
      cargo
      cargo-edit
      clippy
      linuxPackages_latest.perf
      rust-analyzer
      rustfmt
    ];
  }
