use std::io::Read;
use std::{fs::File, path::Path};

use crate::{errors::ErrorExitCode, KeepacCliError, SubcommandResult};

pub struct ShowCommandOptions {
    /// An optional version to show the changes for
    version: Option<String>,
}

pub fn show(path: &Path) -> SubcommandResult {
    let Some(changelog_path) = keepac::find::nearest_changelog_path(path) else {
        return Err(KeepacCliError {
            message: String::from("Failed to find CHANGELOG.md"),
            exit_code: ErrorExitCode::ChangelogNotFound,
        });
    };

    let mut changelog_file = File::open(changelog_path)?;
    let mut changelog_source = String::new();
    changelog_file.read_to_string(&mut changelog_source)?;

    // TODO: Actually highlight
    println!("{}", changelog_source);

    Ok(())
}
