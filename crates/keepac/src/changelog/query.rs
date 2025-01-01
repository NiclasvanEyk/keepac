use tree_sitter::{Query, QueryCursor, QueryMatches};

use crate::Changelog;

pub struct ChangelogQueryCursor<'c> {
    pub(crate) cursor: QueryCursor,
    pub(crate) query: Query,
    pub(crate) changelog: &'c Changelog<'c>,
}

impl<'c> ChangelogQueryCursor<'c> {
    pub fn matches(&'c mut self) -> QueryMatches<'_, '_, &[u8], &[u8]> {
        self.cursor.matches(
            &self.query,
            self.changelog.tree.root_node(),
            self.changelog.source.as_bytes(),
        )
    }
}
