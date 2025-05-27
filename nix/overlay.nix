{ pkgs, ... }:

(final: prev: {
  build = pkgs.callPackage ./build.nix { };
  format = pkgs.callPackage ./format.nix { };
})
