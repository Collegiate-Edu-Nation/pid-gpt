{ pkgs, ... }:

{
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "pid-gpt";
    version = "0.1.0";
    src = ../.;

    cargoHash = "sha256-0lozsvMiLB2OE4kQO9DJyQhqaT1IEu1D5yO77jIHFx0=";
    useFetchCargoVendor = true;
  };
}
