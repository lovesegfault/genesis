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
    , wayland
    }: naersk.buildPackage {
      name = "voyager";
      src = gitignoreSource ./.;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [ wayland xorg.libX11 ];
    }
  )
  args
