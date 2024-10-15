use dialoguer::{Confirm, Input, MultiSelect, Select};

pub fn run() {
    // TODO: prompt `(hit enter to use the current directory)`.
    let path: String = Input::new()
        .with_prompt("Where should we create your project?")
        .default(".".to_owned())
        .interact_text()
        .unwrap();

    // TODO: check if directory is empty. Prompt if not.

    // TODO: prompt `(hit enter to use the directory name)`.
    let name: String = Input::new()
        .with_prompt("What would you like to name your project?")
        .default(path)
        .interact_text()
        .unwrap();

    // TODO: prompt `(hit enter to use the directory name)`.
    let description: String = Input::new()
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

    let lint_groups = vec!["pedantic", "nursery", "restriction"];

    let extra_lint_groups = MultiSelect::new()
        .with_prompt("Would you like extra Clippy lint groups?")
        .items(&lint_groups)
        .interact()
        .unwrap();

    let compiler_types = vec!["stable", "beta", "nightly"];

    let selected_compiler_type = Select::new()
        .with_prompt("Which Rust compiler would you like to use?")
        .default(0)
        .items(&compiler_types)
        .interact()
        .unwrap();

    let has_git_repo = Confirm::new()
        .with_prompt("Would you like to initialize a git repository?")
        .default(true)
        .interact()
        .unwrap();
}
