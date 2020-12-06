{ ... }@args:
let
  pkgs = import ./nix;
  genesis = { naersk, gitignoreSource }: naersk.buildPackage {
    pname = "genesis";
    src = gitignoreSource ./.;
  };
in
pkgs.callPackage genesis args
