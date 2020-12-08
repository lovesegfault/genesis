let
  pkgs = import ./nix;
  genesis = import ./. { };
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

    niv
    nixpkgs-fmt
  ] ++ genesis.buildInputs ++ genesis.nativeBuildInputs;
}
