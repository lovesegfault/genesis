{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:figsoda/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
        naerskBuild = (naersk.lib.${system}.override {
          inherit (fenixPkgs.minimal) cargo rustc;
        }).buildPackage;
      in
      {
        defaultPackage = naerskBuild { src = ./.; };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo-edit
            nixpkgs-fmt
          ] ++ (with fenixPkgs; [ default.toolchain rust-analyzer ]);
        };
      });
}
