let
  sources = import ./sources.nix;
  pkgs = import <nixpkgs>;
  lib = import <nixpkgs/lib>;
in
  pkgs {
    overlays = [
      (self: super: { naersk = self.callPackage sources.naersk { }; })
      (self: super: { gitignoreSource = (import sources.gitignore { inherit lib; }).gitignoreSource; })
    ];
  }
