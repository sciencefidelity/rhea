use std::{env, fs, path::Path};

use crate::readme::generate_readme;
use crate::rust::{generate_bin, generate_cargo_toml, generate_lib};
use crate::{nix::generate_flake, Args};
use anyhow::Result;

#[allow(clippy::missing_errors_doc)]
pub fn run(args: &Args) -> Result<()> {
    let root_dir = Path::new(&args.path);
    // TODO: prompt that directory already exists and ask if we should use it.
    if root_dir.exists() {
        if root_dir.read_dir()?.next().is_none() {
            eprintln!("directory '{}' already exists and is not empty", args.path);
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
    fs::write(root_dir.join("flake.nix"), generate_flake(args))?;
    fs::write(root_dir.join("Cargo.toml"), generate_cargo_toml(args))?;
    fs::write(root_dir.join("README.md"), generate_readme(args))?;

    let src_dir = root_dir.join("src");
    fs::create_dir(&src_dir)?;
    if args.bin {
        fs::write(src_dir.join("main.rs"), generate_bin())?;
    } else if args.lib {
        fs::write(src_dir.join("lib.rs"), generate_lib())?;
    }
    Ok(())
}
