use indoc::formatdoc;

use crate::{args::Compiler, Args};

pub fn generate_flake(args: &Args) -> String {
    let rust_bin = match args.compiler {
        Compiler::Stable => {
            r#"(rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            })"#
        }
        Compiler::Beta => {
            r#"(rust-bin.beta.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            })"#
        }
        Compiler::Nightly => {
            r#"(rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            }))"#
        }
    };
    let mut packages = args.packages.clone();
    packages.push("pkg-config".to_owned());
    packages.sort_unstable();
    let packages = packages.join("\n            ");
    formatdoc! {r#"
        {{
          description = "A Rust devshell";

          inputs = {{
            nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
            rust-overlay.url = "github:oxalica/rust-overlay";
            flake-utils.url = "github:numtide/flake-utils";
          }};

          outputs = {{ self, nixpkgs, rust-overlay, flake-utils, ... }}:
            flake-utils.lib.eachDefaultSystem (system:
              let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {{
                  inherit system overlays;
                }};
              in
              with pkgs;
              {{
                devShells.default = mkShell {{
                  buildInputs = [
                    {packages}
                    {rust_bin}
                  ];

                  shellHook = /*bash*/ ''

                  '';
                }};
              }}
            );
        }}
    "#}
}
