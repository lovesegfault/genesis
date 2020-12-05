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
      rust-analyzer
      linuxPackages_latest.perf
    ];
  }
