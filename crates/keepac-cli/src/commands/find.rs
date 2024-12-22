use std::path::Path;

use crate::{ErrorExitCode, KeepacCliError, SubcommandResult};

pub fn find(path: &Path) -> SubcommandResult {
    match keepac::find::nearest_changelog_path(path) {
        Some(changelog_path) => {
            println!("{}", changelog_path.display());
            Ok(())
        }
        None => Err(KeepacCliError {
            message: String::from("Failed to find CHANGELOG.md"),
            exit_code: ErrorExitCode::ChangelogNotFound,
        }),
    }
}
