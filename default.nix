{ lib, pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  nativeBuildInputs = [ pkgs.pkg-config ];
  OPENSSL_NO_VENDOR = 1;
  buildInputs = [ pkgs.openssl ];
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
