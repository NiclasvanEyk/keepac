use tree_sitter::{Query, QueryCursor, Tree};

use crate::changelog::query::ChangelogQueryCursor;

/// Implementations of [From] and [TryFrom] for [Changelog] from various structs.
mod from;

mod query;

pub struct Changelog<'a> {
    pub(crate) source: &'a str,
    pub(crate) tree: Tree,
}

impl<'a> Changelog<'a> {
    pub fn query(&'a self, query: &str) -> ChangelogQueryCursor<'a> {
        ChangelogQueryCursor {
            query: Query::new(&tree_sitter_md::LANGUAGE.into(), query).unwrap(),
            cursor: QueryCursor::new(),
            changelog: self,
        }
    }

    // TODO: Version iterator
}
