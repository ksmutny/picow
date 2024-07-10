use picow::app;
use std::{env, io::Result, path::Path};

fn main() -> Result<()> {
    match env::args().nth(1) {
        Some(arg) if arg == "-version" =>
            print_version(),
        Some(ref file_path) => if exists(file_path) {
            app::start(file_path)
        } else {
            print_file_not_exists(file_path)
        }
        None =>
            print_usage()
    }
}

fn exists(file_path: &str) -> bool {
    Path::new(&file_path).exists()
}

fn print_usage() -> Result<()> {
    print_ok("Usage: picow <file_path>")
}

fn print_version() -> Result<()> {
    print_ok(&format!("picow {}", env!("CARGO_PKG_VERSION")))
}

fn print_file_not_exists(file_path: &str) -> Result<()> {
    print_ok(&format!("File {} does not exist", file_path))
}

fn print_ok(msg: &str) -> Result<()> {
    println!("{}", msg);
    Ok(())
}
