{ ... }@args:
let
  pkgs = import ./nix;
  genesis = { naersk, gitignoreSource }: naersk.buildPackage {
    name = "genesis";
    src = gitignoreSource ./.;
  };
in
pkgs.callPackage genesis args
