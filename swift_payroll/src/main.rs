use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    serve: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}
