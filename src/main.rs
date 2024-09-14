use std::{env, fs, io, path::Path};

mod nix;
mod rust;
use nix::generate_flake;
use rust::{generate_bin, generate_cargo_toml, generate_lib};

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
    let mut args = std::env::args().skip(1);
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
    fs::write(root_dir.join(".envrc"), "use flake\ndotenv\n")?;
    let current_dir = env::current_dir()?;
    fs::write(
        root_dir.join(".env"),
        format!("PROJECT_ROOT={}", current_dir.display()),
    )?;
    fs::write(root_dir.join(".gitignore"), "/target\n/.direnv\n.env\n")?;
    fs::write(root_dir.join("flake.nix"), generate_flake())?;
    fs::write(root_dir.join("Cargo.toml"), generate_cargo_toml(&name))?;
    fs::write(root_dir.join("README.md"), format!("# {}\n", &name))?;

    let src_dir = root_dir.join("src");
    fs::create_dir(&src_dir)?;
    if app_type == AppType::Bin {
        fs::write(src_dir.join("main.rs"), generate_bin())?;
    } else {
        fs::write(src_dir.join("lib.rs"), generate_lib())?;
    }
    Ok(())
}
