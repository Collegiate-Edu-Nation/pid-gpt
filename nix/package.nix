{ pkgs, deps }:

{
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "pid-gpt";
    version = "0.1.0";
    src = ../.;

    buildInputs = deps.build;
    nativeBuildInputs = deps.run;
    cargoHash = "sha256-5nXWoRuw7z+n8BBATwKZNGb53C9MH1+zTZeHTEAK+pg=";
    useFetchCargoVendor = true;
  };
}
