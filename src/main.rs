fn main() {
    if let Err(err) = pix::cli::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
