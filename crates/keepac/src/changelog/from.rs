use crate::find::nearest_changelog_path;
use crate::parse::{parse_markdown, MarkdownParserError};
use crate::Changelog;
use std::io::Read;
use std::{fs::File, path::Path};
use thiserror::Error;

/// Get a Changelog from a string
impl<'s> TryFrom<&'s str> for Changelog<'s> {
    type Error = MarkdownParserError;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Ok(Self {
            source: std::borrow::Cow::Borrowed(value),
            tree: parse_markdown(value)?,
        })
    }
}

/// Get a Changelog from a string
impl TryFrom<String> for Changelog<'_> {
    type Error = MarkdownParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tree = parse_markdown(&value)?;
        Ok(Self {
            source: std::borrow::Cow::Owned(value),
            tree,
        })
    }
}

#[derive(Error, Debug)]
pub enum ChangelogFromFileError {
    #[error("Failed to read the contents of the changelog file")]
    FileError(#[from] std::io::Error),

    #[error("Failed to parse the markdown file")]
    MarkdownParserError(#[from] MarkdownParserError),
}

/// Get a Changelog from a file
impl TryFrom<&mut File> for Changelog<'_> {
    type Error = ChangelogFromFileError;

    fn try_from(value: &mut File) -> Result<Self, Self::Error> {
        let mut source = String::new();
        value.read_to_string(&mut source)?;

        Ok(Changelog::try_from(source)?)
    }
}

#[derive(Error, Debug)]
pub enum ChangelogFromPathError {
    #[error("Nearest changelog could not be found")]
    NoChangelogFound,

    #[error("Failed to open the changelog file")]
    FileError(#[from] std::io::Error),

    #[error("Failed to parse the markdown file")]
    FailedToOpenChangelog(#[from] ChangelogFromFileError),
}

pub(crate) fn changelog_try_from_nearest(
    value: &Path,
) -> Result<Changelog, ChangelogFromPathError> {
    let Some(nearest) = nearest_changelog_path(value) else {
        return Err(ChangelogFromPathError::NoChangelogFound);
    };

    let mut changelog_file = File::open(nearest)?;

    Ok(Changelog::try_from(&mut changelog_file)?)
}
