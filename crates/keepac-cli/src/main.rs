pub mod commands;

use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "keepac provides useful tools for working with changelogs.",
    long_about = "You can show, search, compare or add new changes from anywhere within your project, without the needing to open up any editors and manually inserting new markdown sections."
)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add {},
    Change {},
    Deprecate {},
    Diff {},
    Edit {},
    Find {},
    Fix {},
    Init {},
    Insert {},
    Release {},
    Remove {},
    Search {},
    Secure {},
    Show {},
    Versions {},
    Yank {},
}

type SubcommandResult = Result<(), anyhow::Error>;

fn run(cli: Cli, path: &Path) -> SubcommandResult {
    let command = cli.command.unwrap_or(Command::Show {});
    match command {
        /* change */ Command::Add {} => todo!(),
        /* change */ Command::Change {} => todo!(),
        /* change */ Command::Deprecate {} => todo!(),
        Command::Diff {} => todo!(),
        Command::Edit {} => commands::edit(path),
        Command::Find {} => commands::find(path),
        /* change */ Command::Fix {} => todo!(),
        Command::Init {} => commands::init(path),
        Command::Insert {} => todo!(),
        Command::Release {} => todo!(),
        /* change */ Command::Remove {} => todo!(),
        Command::Search {} => todo!(),
        /* change */ Command::Secure {} => todo!(),
        Command::Show {} => commands::show(path),
        Command::Versions {} => commands::versions(path),
        Command::Yank {} => todo!(),
    }
}

fn main() {
    let cwd = std::env::current_dir().unwrap();

    let result = run(Cli::parse(), &cwd);

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}
