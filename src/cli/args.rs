use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version = "v0.1.0-alpha", about = "a very grainy package manager", long_about = "a package manager designed for the grainOS linux distribution")]
#[command(arg_required_else_help(true))]
pub struct Args {
    // TODO: add arguments here
}