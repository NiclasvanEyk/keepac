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

fn main() {
    let args = Cli::parse();

    let command = args.command.unwrap_or(Command::Show {});
    match command {
        Command::Add {} => todo!(),
        Command::Change {} => todo!(),
        Command::Deprecate {} => todo!(),
        Command::Diff {} => todo!(),
        Command::Edit {} => todo!(),
        Command::Find {} => todo!(),
        Command::Fix {} => todo!(),
        Command::Init {} => todo!(),
        Command::Insert {} => todo!(),
        Command::Release {} => todo!(),
        Command::Remove {} => todo!(),
        Command::Search {} => todo!(),
        Command::Secure {} => todo!(),
        Command::Show {} => todo!(),
        Command::Yank {} => todo!(),
    }
}
