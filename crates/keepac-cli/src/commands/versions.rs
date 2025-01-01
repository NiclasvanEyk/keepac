use std::{fs::File, io::Read, path::Path};

use keepac::parse::parse_versions;
use keepac::Changelog;

use crate::{errors::ErrorExitCode, KeepacCliError, SubcommandResult};

pub(crate) fn versions(path: &Path) -> SubcommandResult {
    let changelog = Changelog::nearest(path)?;
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
