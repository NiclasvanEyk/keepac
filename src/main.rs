mod cli;
mod commands;
mod finder;
mod parse;

use clap::Parser;
use cli::{ChangelogCli, Commands};
use commands::find;

fn main() {
    let cli = ChangelogCli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Find {} => find(),
    }
}
