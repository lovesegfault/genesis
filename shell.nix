let
  pkgs = import ./nix;
in
pkgs.mkShell {
  pname = "genesis";
  buildInputs = with pkgs; [
    niv
    nixpkgs-fmt
  ];
}
