/// Functions related to finding the CHANGELOG.md
use std::path::{Path, PathBuf};

/// Iterates upwards the directory tree, until a CHANGELOG.md file is found.CHANGELOG
///
/// Right now, this is case-sensitive to improve performance.
pub fn nearest_changelog_path(origin: &Path) -> Option<PathBuf> {
    let mut current_directory = origin;
    loop {
        if let Some(changelog_path) = get_changelog_in(current_directory) {
            return Some(changelog_path);
        };

        match current_directory.parent() {
            Some(parent) => current_directory = parent,
            None => return None,
        }
    }
}

/// Checks whether there is a CHANGELOG.md in the current [directory].
fn get_changelog_in(directory: &Path) -> Option<PathBuf> {
    let changelog_path = directory.join("CHANGELOG.md");
    if changelog_path.is_file() {
        Some(changelog_path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, File};

    use tempdir::TempDir;

    use super::*;

    #[test]
    fn it_finds_if_present_in_the_current_directory() {
        let dir = TempDir::new("it_finds_if_present_in_the_current_directory").unwrap();
        let path = dir.path();
        let actual_changelog_path = path.join("CHANGELOG.md");
        File::create(&actual_changelog_path).unwrap();

        let found_changelog_path = nearest_changelog_path(path).unwrap();
        assert_eq!(actual_changelog_path, found_changelog_path);
    }

    #[test]
    fn it_finds_if_present_in_the_parent_directory() {
        let root_dir = TempDir::new("it_finds_if_present_in_the_parent_directory").unwrap();
        let root_dir_path = root_dir.path();

        let actual_changelog_path = root_dir_path.join("CHANGELOG.md");
        File::create(&actual_changelog_path).unwrap();

        let child_dir_path = root_dir_path.join("child");
        create_dir(&child_dir_path).unwrap();

        let found_changelog_path = nearest_changelog_path(&child_dir_path).unwrap();
        assert_eq!(actual_changelog_path, found_changelog_path);
    }

    #[test]
    fn it_actually_chooses_the_nearest_changelog() {
        let root_dir = TempDir::new("it_finds_if_present_in_the_parent_directory").unwrap();
        let root_dir_path = root_dir.path();

        let parent_changelog_path = root_dir_path.join("CHANGELOG.md");
        File::create(parent_changelog_path).unwrap();

        let child_dir_path = root_dir_path.join("child");
        create_dir(&child_dir_path).unwrap();

        let child_changelog_path = child_dir_path.join("CHANGELOG.md");
        File::create(&child_changelog_path).unwrap();

        let found_changelog_path = nearest_changelog_path(&child_dir_path).unwrap();
        assert_eq!(child_changelog_path, found_changelog_path);
    }

    #[test]
    fn it_can_return_nothing_if_no_changelog_is_present() {
        let root_dir = TempDir::new("it_can_return_nothing_if_no_changelog_is_present").unwrap();
        let found_changelog_path = nearest_changelog_path(root_dir.path());
        assert!(found_changelog_path.is_none());
    }
}
