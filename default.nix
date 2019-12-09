{ pkgs? import <nixpkgs> { config = {}; },
  lib? pkgs.lib,
  cargo? pkgs.cargo,
  nix? pkgs.nix,
  makeWrapper? pkgs.makeWrapper,
  callPackage? pkgs.callPackage,
  darwin? pkgs.darwin,
  stdenv? pkgs.stdenv,
  defaultCrateOverrides? pkgs.defaultCrateOverrides}:

let cargo_nix = callPackage ./Cargo.nix {};
    crate2nix = cargo_nix.rootCrate.build.override {
      crateOverrides = defaultCrateOverrides // {
        rsdarksky = attrs: {
        };
      };
    };
in pkgs.symlinkJoin {
  name = "rsdarksky";
  paths = [ crate2nix ];
}
