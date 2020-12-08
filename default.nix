let
  pkgs = import ./nix;
in
pkgs.buildEnv {
  name = "genesis";
  paths = [
    (import ./dada { })
    (import ./voyager { })
  ];
}
