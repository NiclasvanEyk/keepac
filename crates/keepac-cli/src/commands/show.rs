use std::path::Path;

use keepac::Changelog;
use termcolor::{BufferWriter, ColorChoice};

use crate::SubcommandResult;

#[derive(Debug, clap::Args)]
pub struct ShowCommandOptions {
    /// An optional version to show the changes for
    version: Option<String>,

    /// An optional explicit path to a changelog file to use
    #[arg(short, long)]
    file: Option<String>,
}

impl Default for ShowCommandOptions {
    fn default() -> Self {
        ShowCommandOptions {
            version: None,
            file: None,
        }
    }
}

pub fn show(path: &Path, options: ShowCommandOptions) -> SubcommandResult {
    let changelog = match options.file {
        Some(file) => Changelog::try_from_file_at(file)?,
        None => Changelog::nearest(path)?,
    };

    let writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = writer.buffer();
    keepac::highlight::render(
        &mut buffer,
        std::str::from_utf8(changelog.document.source.as_ref())?,
    )?;
    writer.print(&buffer)?;
    Ok(())
}
