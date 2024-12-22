pub mod commands;

use std::{error::Error, fmt::Display, path::Path};

use clap::{Parser, Subcommand};

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
    Yank {},
}

#[derive(Debug, Clone, Copy)]
enum ErrorExitCode {
    Unknown = 1,
    ChangelogNotFound = 2,
}

#[derive(Debug)]
struct KeepacCliError {
    pub message: String,
    pub exit_code: ErrorExitCode,
}

impl Display for KeepacCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for KeepacCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<anyhow::Error> for KeepacCliError {
    fn from(value: anyhow::Error) -> Self {
        KeepacCliError {
            message: format!("{}", value),
            exit_code: ErrorExitCode::Unknown,
        }
    }
}

impl From<std::io::Error> for KeepacCliError {
    fn from(value: std::io::Error) -> Self {
        KeepacCliError {
            message: format!("{}", value),
            exit_code: ErrorExitCode::Unknown,
        }
    }
}

type SubcommandResult = Result<(), KeepacCliError>;

fn run(cli: Cli, path: &Path) -> SubcommandResult {
    let command = cli.command.unwrap_or(Command::Show {});
    match command {
        Command::Add {} => todo!(),
        Command::Change {} => todo!(),
        Command::Deprecate {} => todo!(),
        Command::Diff {} => todo!(),
        Command::Edit {} => todo!(),
        Command::Find {} => commands::find(path),
        Command::Fix {} => todo!(),
        Command::Init {} => commands::init(path),
        Command::Insert {} => todo!(),
        Command::Release {} => todo!(),
        Command::Remove {} => todo!(),
        Command::Search {} => todo!(),
        Command::Secure {} => todo!(),
        Command::Show {} => todo!(),
        Command::Yank {} => todo!(),
    }
}

fn main() {
    let cwd = std::env::current_dir().unwrap();

    if let Err(err) = run(Cli::parse(), &cwd) {
        eprintln!("{}", err);
        std::process::exit(err.exit_code as i32);
    };
}
