use std::{borrow::Cow, path::Path};

use tree_sitter::{Query, QueryCursor, Tree};

use crate::changelog::query::ChangelogQueryCursor;

use self::from::{changelog_try_from_nearest, ChangelogFromPathError};

/// Implementations of [From] and [TryFrom] for [Changelog] from various structs.
mod from;

mod query;

pub struct Changelog<'a> {
    pub source: Cow<'a, str>,
    pub(crate) tree: Tree,
}

impl<'a> Changelog<'a> {
    /// Tries to find and open a changelog near [path].
    /// If none is found, it recursively hikes the directory tree until a `CHANGELOG.md` file is
    /// found.
    pub fn nearest(path: &Path) -> Result<Changelog, ChangelogFromPathError> {
        changelog_try_from_nearest(path)
    }

    pub fn query(&'a self, query: &str) -> ChangelogQueryCursor<'a> {
        ChangelogQueryCursor {
            query: Query::new(&tree_sitter_md::LANGUAGE.into(), query).unwrap(),
            cursor: QueryCursor::new(),
            changelog: self,
        }
    }

    // TODO: Version iterator
}
