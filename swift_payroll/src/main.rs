use clap::Parser;
mod cli;

fn main() {
    let args = crate::cli::args::CliArgs::parse();
    if let Err(e) = cli::run(&args) {
        eprintln!("Error: {}", e);
    }
}
