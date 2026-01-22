use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to scan
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("Starting scan on: {}", args.path);
    println!("Scanning engine initialized...");

    // Todo: specific scanning logic here
}
