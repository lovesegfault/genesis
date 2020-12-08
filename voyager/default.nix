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
    , vulkan-loader
    , wayland
    , xorg
    }: naersk.buildPackage rec {
      name = "voyager";
      src = gitignoreSource ./.;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [
        libxkbcommon
        vulkan-loader
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
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
