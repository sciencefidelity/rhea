use anyhow::Result;

fn main() -> Result<()> {
    if let Err(e) = rhea::run(rhea::get_args()?) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
    Ok(())
}
