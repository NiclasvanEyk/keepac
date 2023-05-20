use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Find {},
    List {},
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ChangelogCli {
    #[command(subcommand)]
    pub command: Commands,
}
