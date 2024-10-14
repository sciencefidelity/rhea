use std::fmt::{self, Display};

use clap::{builder::EnumValueParser, Arg, ArgAction, Command, ValueEnum};
use serde::{Deserialize, Serialize};

/// Possible values allowed for the `--edition` CLI flag.
pub const RUST_EDITIONS: [&str; 4] = ["2015", "2018", "2021", "2024"];

#[derive(Clone, Debug, Default, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Compiler {
    #[default]
    Stable,
    Beta,
    Nightly,
}

#[derive(Clone, Debug, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LintGroup {
    Pedantic,
    Nursery,
    Restriction,
}

impl Display for LintGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub edition: &'static str,
    pub compiler: Compiler,
    pub name: String,
    pub description: String,
    pub bin: bool,
    pub lib: bool,
    pub git: bool,
    pub packages: Vec<String>,
    pub lint_groups: Vec<LintGroup>,
}

impl Args {
    #[must_use]
    pub fn get() -> Self {
        let matches = Command::new("rhea")
            .bin_name("rhea")
            .styles(CLAP_STYLING)
            .version("0.1.0")
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
                Arg::new("compiler")
                    .long("compiler")
                    .value_name("COMPILER")
                    .value_parser(EnumValueParser::<Compiler>::new())
                    .help("Which Rust compiler to use"),
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
            .arg(
                Arg::new("git")
                    .long("git")
                    .action(ArgAction::SetTrue)
                    .help("Initialize a git repository"),
            )
            .arg(
                Arg::new("packages")
                    .long("packages")
                    .value_name("PACKAGE")
                    .help("Extra environment packages")
                    .num_args(1..),
            )
            .arg(
                Arg::new("lints")
                    .long("lints")
                    .value_name("LINTS")
                    .value_parser(EnumValueParser::<LintGroup>::new())
                    .help("Extra lint groups")
                    .num_args(1..),
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
        let compiler = matches
            .get_one("compiler")
            .unwrap_or(&Compiler::default())
            .clone();
        let (mut bin, mut lib) = (true, false);
        if matches.get_flag("lib") {
            bin = false;
            lib = true;
        }
        let git = matches.get_flag("git");
        let packages = matches
            .get_many("packages")
            .unwrap_or_default()
            .cloned()
            .collect();
        let lint_groups = matches
            .get_many("lints")
            .unwrap_or_default()
            .cloned()
            .collect();

        Self {
            path,
            edition,
            compiler,
            name,
            description,
            bin,
            lib,
            git,
            packages,
            lint_groups,
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
