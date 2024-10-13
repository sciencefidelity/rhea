use std::io;

use clap::Parser;
use rhea::Args;

fn main() -> io::Result<()> {
    if let Err(e) = rhea::run(Args::parse()) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
    Ok(())
}
