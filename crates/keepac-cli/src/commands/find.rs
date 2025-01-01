use std::path::Path;

use anyhow::anyhow;

use crate::SubcommandResult;

pub fn find(path: &Path) -> SubcommandResult {
    match keepac::find::nearest_changelog_path(path) {
        Some(changelog_path) => {
            println!("{}", changelog_path.display());
            Ok(())
        }
        None => Err(anyhow!("Failed to find CHANGELOG.md")),
    }
}
