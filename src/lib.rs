mod nix;
mod rust;

use std::{env, fs, io, path::Path};

use crate::nix::generate_flake;
use crate::rust::{generate_bin, generate_cargo_toml, generate_lib};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// App name
    #[arg(value_name = "NAME")]
    name: String,

    /// Create a binary project (default)
    #[arg(long)]
    bin: bool,

    /// Create a library project
    #[arg(long, conflicts_with("bin"))]
    lib: bool,
}

pub fn run(args: Args) -> io::Result<()> {
    let root_dir = Path::new(&args.name);
    // TODO: prompt that directory already exists and ask if we should use it.
    if root_dir.exists() {
        if root_dir.read_dir()?.next().is_none() {
            eprintln!("directory '{}' already exists and is not empty", args.name);
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
    fs::write(root_dir.join("Cargo.toml"), generate_cargo_toml(&args.name))?;
    fs::write(root_dir.join("README.md"), format!("# {}\n", &args.name))?;

    let src_dir = root_dir.join("src");
    fs::create_dir(&src_dir)?;
    if args.lib {
        fs::write(src_dir.join("main.rs"), generate_bin())?;
    } else {
        fs::write(src_dir.join("lib.rs"), generate_lib())?;
    }
    Ok(())
}
