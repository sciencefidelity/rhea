{
  description = "A Rust devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        name = manifest.name;
      in
      rec
      {
        packages = {
          ${name} = pkgs.callPackage ./default.nix { };
          default = packages.${name};
        };

        apps.${name} = {
          type = "app";
          program = "${self.packages.${system}.runme}/bin/${name}";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            })
          ];

          shellHook = /*bash*/ ''
          '';
        };
      }
    );
}

