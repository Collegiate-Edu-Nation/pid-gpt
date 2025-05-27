{ pkgs }:

{
  dev = with pkgs; [
    rustc
    cargo
    rust-analyzer
    rustfmt
    taplo
    clippy
    build
    format
  ];
}
