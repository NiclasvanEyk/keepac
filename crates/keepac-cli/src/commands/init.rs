use anyhow::anyhow;
use keepac::render::render;
use std::{fs::File, io::Write, path::Path};

use crate::SubcommandResult;

static TEMPLATE: &str = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
"#;

pub(crate) fn init(path: &Path) -> SubcommandResult {
    let changelog_path = path.join("CHANGELOG.md");
    if changelog_path.exists() {
        return Err(anyhow!("CHANGELOG.md already exists").into());
    }

    let mut changelog = File::create_new(&changelog_path)?;
    changelog.write_all(TEMPLATE.as_bytes())?;

    // TODO: Print _highlighted_ to console
    println!(
        "Initialized empty changelog at {}:",
        changelog_path.display()
    );
    render(TEMPLATE);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempdir::TempDir;

    use super::*;

    #[test]
    fn it_inits_if_empty() {
        let dir = TempDir::new("it_inits_if_empty").unwrap();
        let path = dir.path();

        let result = init(path);
        assert!(result.is_ok());

        let changelog_path = path.join("CHANGELOG.md");
        assert!(changelog_path.is_file());
    }

    #[test]
    fn it_fails_if_changelog_already_exists() {
        let dir = TempDir::new("it_fails_if_changelog_already_exists").unwrap();
        let path = dir.path();

        let changelog_path = path.join("CHANGELOG.md");
        File::create(changelog_path).unwrap();

        let result = init(path);
        assert!(result.is_err());
    }
}
