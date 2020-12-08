{ ... }@args:
let
  pkgs = import ./nix;
  genesis = { naersk
  , gitignoreSource
  , xorg
  , pkg-config
  }: naersk.buildPackage {
    name = "genesis";
    src = gitignoreSource ./.;

    nativeBuildInputs = [ pkg-config ];
    buildInputs = [ xorg.libX11 ];
  };
in
pkgs.callPackage genesis args
