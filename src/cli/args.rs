use clap::{command, Parser, Subcommand};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version = "v0.1.0-alpha", about = "a very grainy package manager", long_about = "a package manager designed for the grainOS linux distribution")]
#[command(arg_required_else_help(true))]
pub struct Args {
    #[command(subcommand)]
    subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    #[command(about = "install a package")]
    #[command(alias = "i")]
    Install(InstallArgs),

    #[command(about = "remove a package")]
    #[command(alias = "r")]
    Remove(RemoveArgs),

    #[command(about = "update the package list")]
    #[command(alias = "u")]
    Update(UpdateArgs),

    #[command(about = "upgrade installed packages")]
    #[command(alias = "up")]
    Upgrade(UpgradeArgs),
}

#[derive(Parser, Debug)]
#[command(about = "install a package")]
pub struct InstallArgs {
    name: String,
}

#[derive(Parser, Debug)]
#[command(about = "remove a package")]
pub struct RemoveArgs {
    name: String,
}

#[derive(Parser, Debug)]
#[command(about = "update the package list")]
pub struct UpdateArgs {}

#[derive(Parser, Debug)]
#[command(about = "upgrade installed packages")]
pub struct UpgradeArgs {}
