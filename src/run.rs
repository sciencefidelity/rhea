use std::{env, fs, path::PathBuf};

use anyhow::Result;
use dialoguer::Confirm;

use crate::config::{self, Config};
use crate::interactive;
use crate::readme::generate_readme;
use crate::rust::{generate_bin, generate_cargo_toml, generate_lib};
use crate::{nix::generate_flake, Args};

#[allow(clippy::missing_errors_doc)]
pub fn run(args: &mut Args) -> Result<()> {
    let config = config::read()?;

    let root_dir = if args.interactive {
        interactive::run(args)?
    } else {
        create_root_dir(args)?
    };
    create_files(args, &config, &root_dir)?;
    create_git_repo(args, &root_dir)?;
    Ok(())
}

pub fn create_root_dir(args: &Args) -> Result<PathBuf> {
    let root_dir = if args.path == ".".to_owned() {
        std::env::current_dir()?
    } else {
        PathBuf::from(&args.path)
    };

    // TODO: if we create the same directory twice there is an error `File exists (os error 17)`
    if root_dir.exists() {
        if root_dir.read_dir()?.next().is_some() {
            let confirmation = Confirm::new()
                .with_prompt(format!(
                    "directory '{}' already exists and is not empty",
                    args.path
                ))
                .default(true)
                .interact()
                .unwrap();

            if !confirmation {
                std::process::exit(1);
            }
        }
    } else {
        fs::create_dir(&root_dir)?;
    }
    Ok(root_dir)
}

fn create_files(args: &Args, config: &Config, root_dir: &PathBuf) -> Result<()> {
    fs::write(root_dir.join(".envrc"), "use flake\ndotenv\n")?;
    let current_dir = env::current_dir()?;
    fs::write(
        root_dir.join(".env"),
        format!("PROJECT_ROOT={}", current_dir.display()),
    )?;
    fs::write(root_dir.join(".gitignore"), "/target\n/.direnv\n.env\n")?;
    fs::write(root_dir.join("flake.nix"), generate_flake(args))?;
    fs::write(
        root_dir.join("Cargo.toml"),
        generate_cargo_toml(args, config),
    )?;
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

fn create_git_repo(args: &Args, root_dir: &PathBuf) -> Result<()> {
    if args.git {
        let _repo = git2::Repository::init(&root_dir)?;
        // TODO: git add .
        // let mut index = repo.index()?;
        // index.add_all(
        //     &[fs::canonicalize(root_dir).unwrap()],
        //     git2::IndexAddOption::DEFAULT,
        //     None,
        // )?;
        // index.write_tree()?;
        // index.write()?;
    }
    Ok(())
}
