{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rust-analyzer
    rustc
    rustfmt
    rustPackages.clippy
  ];
  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
}
