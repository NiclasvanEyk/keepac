use std::path::Path;

use crate::markdown::MarkdownDocument;

use self::from::{changelog_try_from_nearest, ChangelogFromPathError};

/// Implementations of [From] and [TryFrom] for [Changelog] from various structs.
mod from;

pub struct Changelog<'a> {
    pub document: MarkdownDocument<'a>,
}

impl<'a> Changelog<'a> {
    /// Tries to find and open a changelog near [path].
    /// If none is found, it recursively hikes the directory tree until a `CHANGELOG.md` file is
    /// found.
    pub fn nearest(path: &Path) -> Result<Changelog, ChangelogFromPathError> {
        changelog_try_from_nearest(path)
    }

    // TODO: Version section iterator, maybe even structured in some way?
}
