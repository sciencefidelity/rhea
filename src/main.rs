use std::{env, fs, io, path::Path};

#[derive(Debug, PartialEq)]
enum AppType {
    Lib,
    Bin,
}

fn main() -> io::Result<()> {
    match run() {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("{err:?}");
            std::process::abort();
        }
    }
}

fn run() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let name = args.next().expect("missing argument 'name'");
    let mut app_type = AppType::Bin;
    if let Some(t) = args.next() {
        match t.as_str() {
            "--bin" => app_type = AppType::Bin,
            "--lib" => app_type = AppType::Lib,
            _ => {
                eprintln!("unknown type {app_type:?}");
                std::process::abort();
            }
        }
    }

    let root_dir = Path::new(&name);
    // TODO: prompt that directory already exists and ask if we should use it.
    if root_dir.exists() {
        if root_dir.read_dir()?.next().is_none() {
            eprintln!("directory '{name}' already exists and is not empty");
            std::process::abort();
        }
    } else {
        fs::create_dir(root_dir)?;
    }
    fs::write(root_dir.join(".envrc"), "use flake\ndotenv")?;
    let current_dir = env::current_dir()?;
    fs::write(
        root_dir.join(".env"),
        format!("PROJECT_ROOT={}", current_dir.display()),
    )?;
    fs::write(root_dir.join(".gitignore"), "/target\n/.direnv\n.env")?;
    fs::write(root_dir.join("flake.nix"), generate_flake())?;
    fs::write(root_dir.join("Cargo.toml"), generate_cargo_toml(&name))?;
    fs::write(root_dir.join("README.md"), format!("# {}", &name))?;

    let src_dir = root_dir.join("src");
    fs::create_dir(&src_dir)?;
    if app_type == AppType::Bin {
        fs::write(src_dir.join("main.rs"), generate_bin())?;
    } else {
        fs::write(src_dir.join("lib.rs"), generate_lib())?;
    }
    Ok(())
}

fn generate_flake() -> String {
    String::from(
        r#"{
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
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
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
}"#,
    )
}

fn generate_cargo_toml(name: &String) -> String {
    // TODO: get author, GitHub username and email programmatically.
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
authors = ["Matt Cook <matt@mattcook.dev>"]
description = ""
readme = "README.md"
repository = "https://github.com/sciencefidelity/{name}"
license = "MIT or Apache-2.0"

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
enum_glob_use = "deny"
pedantic = {{ level = "deny", priority = 1 }}
nursery = {{ level = "deny", priority = 2 }}
unwrap_used = "deny"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"

[dependencies]
    "#,
    )
}

fn generate_bin() -> String {
    String::from(
        r#"fn main() {
    println!("Hello, world!");
}"#,
    )
}

fn generate_lib() -> String {
    String::from(
        "pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}",
    )
}
