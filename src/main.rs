fn main() {
    if let Err(error) = spongecrab::run() {
        eprintln!("{error}");
        std::process::exit(1)
    }
}

