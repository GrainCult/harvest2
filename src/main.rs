use clap::Parser;
use cli::args::Args;

#[cfg(test)]
mod tests;
mod cli;

fn main() {
    // parse cli
    let cli = Args::parse();
}
