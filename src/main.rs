use rhea::Args;

fn main() {
    if let Err(e) = rhea::run(&mut Args::get()) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
