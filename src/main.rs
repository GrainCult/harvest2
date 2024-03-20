mod cli;
use clap::Parser;
use cli::args::Args;

fn main() {
    // parse cli
    let cli = Args::parse();
}
