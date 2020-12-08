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
      name = "voyager";
      src = gitignoreSource ./.;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [ xorg.libX11 ];
    }
  )
  args
