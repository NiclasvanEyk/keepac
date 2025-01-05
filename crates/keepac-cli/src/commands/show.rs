use std::path::Path;

use keepac::Changelog;

use crate::SubcommandResult;

// pub struct ShowCommandOptions {
//     /// An optional version to show the changes for
//     version: Option<String>,
// }

pub fn show(path: &Path) -> SubcommandResult {
    let changelog = Changelog::nearest(path)?;

    // TODO: Actually highlight
    println!("{}", changelog);
    Ok(())
}
