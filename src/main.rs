use clap::Parser;

/// Rust rewrite of xxd
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Wether to revert
    #[clap(short, long)]
    revert: bool,
}

fn main() {
    let args = Args::parse();

    println!(
        "You chose to {}!",
        if args.revert { "revert" } else { "not revert" }
    );
}
