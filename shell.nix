let
  pkgs = import ./nix;
in
pkgs.mkShell {
  name = "genesis";
  buildInputs = with pkgs; [
    niv
    nixpkgs-fmt
  ];
}
