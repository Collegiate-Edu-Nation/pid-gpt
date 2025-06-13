{ pkgs }:

{
  build = with pkgs; [
    openssl
  ];

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

  run = with pkgs; [
    pkg-config
  ];
}
