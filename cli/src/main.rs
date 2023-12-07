use clap::Parser;
use cli::{Cli, Commands};
use git::Git;
use colored::*;

use crate::commands::beam;

mod cli;
mod commands;

fn main() {
    let git_version = Git::get_version();
    println!("{} {}", "Using git version:".green(), git_version.blue());

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Beam) => beam::run(),
        None => {}
    }
}
