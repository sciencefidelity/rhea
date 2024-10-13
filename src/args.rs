use clap::{Arg, ArgAction, Command};

/// Possible values allowed for the `--edition` CLI flag.
pub const RUST_EDITIONS: [&str; 4] = ["2015", "2018", "2021", "2024"];

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub edition: &'static str,
    pub name: String,
    pub description: String,
    pub bin: bool,
    pub lib: bool,
}

impl Args {
    #[must_use]
    pub fn get() -> Self {
        let matches = Command::new("rhea")
            .bin_name("rhea")
            .styles(CLAP_STYLING)
            .version("0.1.0")
            .author("Matt Cook <matt@mattcook.dev>")
            .about("Create a new Cargo package at <path> with Nix")
            .arg(Arg::new("path").value_name("PATH"))
            .arg(
                Arg::new("edition")
                    .long("edition")
                    .value_name("YEAR")
                    .value_parser(RUST_EDITIONS)
                    .help("Rust edition"),
            )
            .arg(
                Arg::new("name")
                    .long("name")
                    .value_name("NAME")
                    .help("Set the package name (defaults to the directory name)"),
            )
            .arg(
                Arg::new("description")
                    .long("description")
                    .value_name("DESCRIPTION")
                    .help("Set the package description"),
            )
            .arg(
                Arg::new("bin")
                    .long("bin")
                    .action(ArgAction::SetTrue)
                    .help("Use the binary template (default)"),
            )
            .arg(
                Arg::new("lib")
                    .long("lib")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("bin")
                    .help("Use the library template"),
            )
            .get_matches();

        let path = matches
            .get_one("path")
            .cloned()
            .unwrap_or_else(|| ".".to_owned());
        let name = matches
            .get_one("name")
            .cloned()
            .unwrap_or_else(|| path.clone());
        let description = matches
            .get_one("description")
            .cloned()
            .unwrap_or(String::new());
        let edition = matches.get_one("edition").copied().unwrap_or("2021");
        let (mut bin, mut lib) = (true, false);
        if matches.get_one("lib") == Some(&true) {
            bin = false;
            lib = true;
        }

        Self {
            path,
            edition,
            name,
            description,
            bin,
            lib,
        }
    }
}

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
