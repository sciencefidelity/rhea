use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use dialoguer::{Confirm, Input, MultiSelect, Select};

use crate::args::{Compiler, LintGroup};
use crate::run::create_root_dir;
use crate::Args;

pub fn run(args: &mut Args) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let current_dir_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // TODO: prompt `(hit enter to use the current directory)`.
    args.path = Input::new()
        .with_prompt("Where should we create your project?")
        .default(current_dir_name)
        .interact_text()
        .unwrap();

    // TODO: check if directory is empty. Prompt if not.
    let root_dir = create_root_dir(args)?;
    let root_dir_name = root_dir.file_name().unwrap().to_str().unwrap().to_owned();

    // TODO: prompt `(hit enter to use the directory name)`.
    args.name = Input::new()
        .with_prompt("What would you like to name your project?")
        .default(root_dir_name)
        .interact_text()
        .unwrap();

    // TODO: prompt `(hit enter to use the directory name)`.
    args.description = Input::new()
        .with_prompt("Describe your project?")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let app_type = vec!["bin", "lib"];

    let selected_app_type = Select::new()
        .with_prompt("Is your project a binary or a library?")
        .default(0)
        .items(&app_type)
        .interact()
        .unwrap();

    if selected_app_type == 1 {
        args.lib = true;
    } else {
        args.bin = true;
    }

    let lint_groups = vec!["pedantic", "nursery", "restriction"];

    let extra_lint_groups = MultiSelect::new()
        .with_prompt("Would you like extra Clippy lint groups?")
        .items(&lint_groups)
        .interact()
        .unwrap();

    for i in extra_lint_groups {
        args.lint_groups.push(LintGroup::from_str(lint_groups[i])?);
    }

    let compiler_types = vec!["stable", "beta", "nightly"];

    let selected_compiler_type = Select::new()
        .with_prompt("Which Rust compiler would you like to use?")
        .default(0)
        .items(&compiler_types)
        .interact()
        .unwrap();

    args.compiler = Compiler::from_str(compiler_types[selected_compiler_type])?;

    args.git = Confirm::new()
        .with_prompt("Would you like to initialize a git repository?")
        .default(true)
        .interact()
        .unwrap();

    Ok(root_dir)
}
