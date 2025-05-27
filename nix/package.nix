{ pkgs, ... }:

{
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "ftc-gpt";
    version = "0.1.0";
    src = ../.;

    cargoHash = "sha256-QT5KhM92A5JuY/Hh+ccwHHEmfYsk8l3xxnAM+1TbYI4=";
    useFetchCargoVendor = true;
  };
}
