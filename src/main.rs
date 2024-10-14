use rhea::Args;

fn main() {
    if let Err(e) = rhea::run(&Args::get()) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
