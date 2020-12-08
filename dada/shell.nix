let
  pkgs = import ../nix;
  self = import ./. { };
in
pkgs.mkShell {
  name = self.name;
  buildInputs = self.buildInputs ++ self.nativeBuildInputs ++ (with pkgs; [
    rust-analyzer
    cargo-edit
  ]);
}
