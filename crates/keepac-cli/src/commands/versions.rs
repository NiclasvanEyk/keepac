use std::{fs::File, io::Read, path::Path};

use keepac::parse::{parse_versions, Changelog};

use crate::{errors::ErrorExitCode, KeepacCliError, SubcommandResult};

pub(crate) fn versions(path: &Path) -> SubcommandResult {
    let Some(changelog_path) = keepac::find::nearest_changelog_path(path) else {
        return Err(KeepacCliError {
            message: String::from("Failed to find CHANGELOG.md"),
            exit_code: ErrorExitCode::ChangelogNotFound,
        });
    };

    let mut changelog_file = File::open(changelog_path)?;
    let mut changelog_source = String::new();
    changelog_file.read_to_string(&mut changelog_source)?;

    // TODO: don't unwrap here
    let changelog = Changelog::try_from(changelog_source.as_str()).unwrap();
    let versions = parse_versions(&changelog);

    let mut last_seen_year: Option<u32> = None;
    for version in versions {
        if version.released_at.is_none() {
            continue;
        }

        let year = version
            .released_at
            .and_then(|released_at| released_at.split('-').next())
            .and_then(|year| year.parse::<u32>().ok());

        if year != last_seen_year {
            println!();
            last_seen_year = year;
        }

        println!(
            "{}  {}",
            version.released_at.unwrap_or("          "),
            version.name
        );
    }

    Ok(())
}
