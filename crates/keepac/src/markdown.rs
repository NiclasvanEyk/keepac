use std::borrow::Cow;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Query, QueryCursor, QueryMatch, QueryMatches, Tree};

use crate::parse::{parse_markdown, MarkdownParserError};

pub struct MarkdownDocument<'md> {
    pub source: Cow<'md, [u8]>,
    pub(crate) tree: Tree,
}

impl<'md> MarkdownDocument<'md> {
    pub fn query_cursor(
        &self,
        query: &str,
    ) -> Result<MarkdownDocumentQueryCursor, tree_sitter::QueryError> {
        Ok(MarkdownDocumentQueryCursor {
            cursor: QueryCursor::new(),
            query: Query::new(&tree_sitter_md::LANGUAGE.into(), query)?,
            document: self,
        })
    }
}

pub struct MarkdownDocumentQueryCursor<'md> {
    cursor: QueryCursor,
    query: Query,
    document: &'md MarkdownDocument<'md>,
}

impl<'md> MarkdownDocumentQueryCursor<'md> {
    pub fn matches(&mut self) -> QueryMatches<'_, '_, &[u8], &[u8]> {
        self.cursor.matches(
            &self.query,
            self.document.tree.root_node(),
            self.document.source.as_ref(),
        )
    }

    pub fn for_each<F: FnMut(&QueryMatch<'_, '_>)>(&mut self, mut callback: F) {
        let mut matches = self.matches();
        while let Some(res) = matches.next() {
            callback(res);
        }
    }

    pub fn captured_nodes(&mut self) -> Vec<Node<'_>> {
        let mut matches = self.matches();
        let mut nodes = Vec::new();
        while let Some(query_match) = matches.next() {
            for capture in query_match.captures {
                nodes.push(capture.node);
            }
        }

        nodes
    }
}

impl<'md> TryFrom<&'md str> for MarkdownDocument<'md> {
    type Error = MarkdownParserError;

    fn try_from(value: &'md str) -> Result<Self, Self::Error> {
        let tree = parse_markdown(value)?;

        Ok(MarkdownDocument {
            source: Cow::Borrowed(value.as_ref()),
            tree,
        })
    }
}

impl<'md> TryFrom<String> for MarkdownDocument<'md> {
    type Error = MarkdownParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tree = parse_markdown(&value)?;

        Ok(MarkdownDocument {
            source: Cow::Owned(value.into()),
            tree,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CHANGELOG: &str = r#"# Changelog

This is a changelog.

## The second version

This is the second version.

## The first version

This is the first version.
"#;

    #[test]
    fn it_can_iterate_results() {
        let document = MarkdownDocument::try_from(EXAMPLE_CHANGELOG).unwrap();
        let mut query = document.query_cursor("(paragraph) @par").unwrap();

        let mut foo: Vec<String> = Vec::new();
        query.for_each(|res| {
            for capt in res.captures {
                foo.push(
                    capt.node
                        .utf8_text(EXAMPLE_CHANGELOG.as_bytes())
                        .unwrap()
                        .into(),
                );
            }
        });

        assert_eq!(foo.get(0).unwrap().trim(), "This is a changelog.");
        assert_eq!(foo.get(1).unwrap().trim(), "This is the second version.");
        assert_eq!(foo.get(2).unwrap().trim(), "This is the first version.");
    }

    #[test]
    pub fn it_can_iterate_nodes() {
        let document = MarkdownDocument::try_from(EXAMPLE_CHANGELOG).unwrap();
        let mut cursor = document.query_cursor("(paragraph) @par").unwrap();

        for node in cursor.captured_nodes() {
            assert_eq!(node.grammar_name(), "paragraph");
        }
    }
}
