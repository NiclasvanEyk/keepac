use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn find_changelog_relative_to_cwd() -> Result<PathBuf, String> {
    let cwd = current_dir();
    if cwd.is_err() {
        return Result::Err(String::from(
            "Could not determine current working directory!",
        ));
    }

    let changelog = find_changelog(&cwd.unwrap());
    if changelog.is_none() {
        return Result::Err(String::from(
            "Could not find a changelog relative to the current working directory!",
        ));
    }

    return Result::Ok(changelog.unwrap());
}

/// Tries to find a file containing a changelog relative to the directory.
///
/// If none is present in the current directory, it will iterate upwards,
/// until it either finds a matching file, or reaches the root directory.
///
/// Only files called `CHANGELOG.md` are seen as valid!
pub fn find_changelog(directory: &Path) -> Option<PathBuf> {
    let mut current_directory = directory;

    loop {
        let changelog_file = current_directory.join("CHANGELOG.md");
        if changelog_file.is_file() {
            return Some(changelog_file);
        }

        let parent = current_directory.parent();
        if parent.is_none() {
            break;
        }

        current_directory = parent.unwrap();
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use std::{
        env,
        fs::{self, DirBuilder, File},
    };
    use uuid::Uuid;

    #[test]
    fn it_finds_changelogs_from_within_nested_directories() {
        within("some/nested/directory", |directory| {
            let changelog_file = directory.join("CHANGELOG.md");
            let mut file = File::create(&changelog_file).expect("Failed to create test file");
            writeln!(file, "Test content").expect("Failed to write to test file");

            assert!(find_changelog(&directory.join("some/nested/directory")).is_some());
        });
    }

    #[test]
    fn it_finds_changelogs_within_the_same_directory() {
        within("", |directory| {
            let changelog_file = directory.join("CHANGELOG.md");
            let mut file = File::create(&changelog_file).expect("Failed to create test file");
            writeln!(file, "Test content").expect("Failed to write to test file");

            assert!(find_changelog(&directory).is_some());
        });
    }

    #[test]
    fn it_sometimes_does_not_find_a_changelog() {
        within("", |directory| {
            assert!(find_changelog(&directory).is_none());
        });
    }

    fn within<Callback: FnOnce(&PathBuf)>(directories: &str, callback: Callback) {
        let temp_dir = env::temp_dir()
            .join("keepac_tests")
            .join(Uuid::new_v4().to_string());

        let path_structure = temp_dir.join(directories);
        DirBuilder::new()
            .recursive(true)
            .create(path_structure)
            .expect("Failed to create directory structure");

        callback(&temp_dir);

        fs::remove_dir_all(&temp_dir).expect("Failed to remove temporary directory");
    }
}
