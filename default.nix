{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
    # minimal openssl rust
    # nativeBuildInputs = [ rustc cargo openssl.dev pkg-config ];
    # extended
    nativeBuildInputs = [ rustup rustc rustfmt cargo cargo-watch just openssl.dev pkg-config ];
}
