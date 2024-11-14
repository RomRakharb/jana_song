{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rust-analyzer
    rustfmt   
    clippy

    bacon
  ];

  shellHook = ''
    clear
    echo " -------------------------------------------"
    echo " | Welcome to Rust Development Environment |"
    echo " -------------------------------------------"
    echo ""
    rustc --version
    cargo --version
    echo ""
  '';
}
