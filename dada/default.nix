{ ... }@args:
let
  pkgs = import ../nix;
in
pkgs.callPackage
  (
    { naersk
    , gitignoreSource
    , xorg
    , pkg-config
    }: naersk.buildPackage {
      name = "dada";
      src = gitignoreSource ./.;
    }
  )
  args
