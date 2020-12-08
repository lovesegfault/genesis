{ ... }@args:
let
  pkgs = import ../nix;
in
pkgs.callPackage
  (
    { stdenv
    , gitignoreSource
    , lib
    , libxkbcommon
    , naersk
    , pkg-config
    , wayland
    , xorg
    }: naersk.buildPackage rec {
      name = "voyager";
      src = gitignoreSource ./.;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [
        libxkbcommon
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
      ];

      singleStep = true;

      libPath = lib.makeLibraryPath buildInputs;

      postInstall = ''
        patchelf \
          --set-interpreter ${stdenv.cc.bintools.dynamicLinker} \
          --set-rpath "${libPath}" \
          $out/bin/${name}
      '';
    }
  )
  args
