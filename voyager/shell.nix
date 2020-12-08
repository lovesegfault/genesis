let
  pkgs = import ../nix;
  super = import ./. { };
in
pkgs.mkShell {
  name = super.name;

  buildInputs = super.buildInputs ++ super.nativeBuildInputs ++ (with pkgs; [
    rust-analyzer
    cargo-edit
    rustfmt
    clippy
  ]);

  LD_LIBRARY_PATH = super.libPath;
}
