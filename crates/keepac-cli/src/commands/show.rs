use std::io::Read;
use std::{fs::File, path::Path};

use keepac::Changelog;

use crate::{errors::ErrorExitCode, KeepacCliError, SubcommandResult};

pub struct ShowCommandOptions {
    /// An optional version to show the changes for
    version: Option<String>,
}

pub fn show(path: &Path) -> SubcommandResult {
    let changelog = Changelog::nearest(path)?;

    // TODO: Actually highlight
    println!("{}", changelog.source);
    Ok(())
}
