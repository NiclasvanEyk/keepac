use std::{fs, process};

use crate::parse::{parse::parse, structs::Changelog};

use super::find::find_from_cwd;

pub fn resolve_changelog() -> Changelog {
    let changelog_path = unwrap_gracefully(find_from_cwd());
    let changelog_contents = unwrap_gracefully(fs::read_to_string(changelog_path));
    let changelog = unwrap_gracefully(parse(&changelog_contents));

    return changelog;
}

pub fn list() {
    let changelog = resolve_changelog();

    if changelog.releases.next.is_some() {
        println!("- Unreleased")
    }

    for release in changelog.releases.past {
        println!("- {} ({})", release.version, release.date)
    }
}

fn unwrap_gracefully<T, U: std::fmt::Display>(result: Result<T, U>) -> T {
    return match result {
        Ok(it) => it,
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    };
}
