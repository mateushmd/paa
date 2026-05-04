{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    rustfmt
    cargo
    clippy
    gcc
  ];
}
