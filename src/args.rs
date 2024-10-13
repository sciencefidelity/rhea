use anyhow::Result;
use clap::{Arg, ArgAction, Command};

/// Possible values allowed for the `--edition` CLI flag.
///
/// This requires a static value due to the way clap works, otherwise I
/// would have built this dynamically.
pub const RUST_EDITIONS: [&'static str; 4] = ["2015", "2018", "2021", "2024"];

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub edition: &'static str,
    pub name: String,
    pub description: String,
    pub bin: bool,
    pub lib: bool,
}

pub fn get_args() -> Result<Args> {
    let matches = Command::new("rhea")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev>")
        .about("Create rust apps in a Nix environment")
        .arg(Arg::new("path").value_name("PATH"))
        .arg(
            Arg::new("edition")
                .long("edition")
                .value_name("YEAR")
                .value_parser(RUST_EDITIONS)
                .help("Rust edition [possible values: 2015, 2018, 2021, 2024]"),
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

    let path = matches.get_one("path").cloned().unwrap_or(".".to_owned());
    let name = matches.get_one("name").cloned().unwrap_or(path.to_owned());
    let description = matches
        .get_one("description")
        .cloned()
        .unwrap_or(String::new());
    let edition = matches.get_one("edition").cloned().unwrap_or("2021");
    let (mut bin, mut lib) = (true, false);
    if let Some(true) = matches.get_one("lib") {
        bin = false;
        lib = true;
    }

    Ok(Args {
        path,
        edition,
        name,
        description,
        bin,
        lib,
    })
}
