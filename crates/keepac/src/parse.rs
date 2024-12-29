use std::collections::HashMap;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor, QueryMatch, QueryMatches, Tree};

pub struct Changelog<'a> {
    pub source: &'a str,
    pub tree: Tree,
}

impl<'a> Changelog<'a> {
    pub fn query(&'a self, query: &str) -> ChangelogQueryCursor<'a> {
        ChangelogQueryCursor {
            query: Query::new(&tree_sitter_md::LANGUAGE.into(), query).unwrap(),
            cursor: QueryCursor::new(),
            changelog: self,
        }
    }
}

pub struct ChangelogQueryCursor<'c> {
    cursor: QueryCursor,
    query: Query,
    changelog: &'c Changelog<'c>,
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

#[derive(Debug)]
pub enum MarkdownParserError {
    FailedToLoadGrammar,
    CouldNotParseTree,
}

impl<'a> TryFrom<&'a str> for Changelog<'a> {
    type Error = MarkdownParserError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser = Parser::new();

        let Ok(_) = parser.set_language(&tree_sitter_md::LANGUAGE.into()) else {
            return Err(MarkdownParserError::FailedToLoadGrammar);
        };

        let Some(tree) = parser.parse(value, None) else {
            return Err(MarkdownParserError::CouldNotParseTree);
        };

        Ok(Self {
            source: value,
            tree,
        })
    }
}

#[derive(Debug)]
pub struct Version<'a> {
    pub name: &'a str,
    pub released_at: Option<&'a str>,
    pub href: Option<&'a str>,
}

pub fn parse_versions<'c>(changelog: &Changelog<'c>) -> Vec<Version<'c>> {
    let mut query =
        changelog.query("(atx_heading (atx_h2_marker) heading_content: (inline) @content)");
    let mut matches = query.matches();

    let mut versions = Vec::new();
    while let Some(m) = matches.next() {
        for c in m.captures {
            let text = c.node.utf8_text(changelog.source.as_bytes()).unwrap();
            let parts: Vec<&str> = text.split(" - ").collect();

            versions.push(Version {
                name: parts[0],
                released_at: parts.get(1).copied(),
                href: None,
            })
        }
    }

    versions
}

pub fn gather_link_defs<'a>(changelog: Changelog<'a>) -> HashMap<&'a str, &'a str> {
    let mut query = changelog
        .query("(link_reference_definition (link_label) @label (link_destination) @destination)");
    let mut matches = query.matches();

    let mut links = HashMap::new();
    while let Some(m) = matches.next() {
        let captures = m.captures;

        let label = captures
            .first()
            .unwrap()
            .node
            .utf8_text(changelog.source.as_bytes())
            .unwrap()
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap();
        let destination = captures
            .get(1)
            .unwrap()
            .node
            .utf8_text(changelog.source.as_bytes())
            .unwrap();

        links.insert(label, destination);
    }

    links
}

#[cfg(test)]
mod tests {
    use super::*;

    static A_CHANGELOG: &str = r#"# Changelog

blah blah blah

## [1.0.1] - 2024-12-12

### Added

- Something cool with `fancy` _annotations_

## [1.0.0] - 2024-12-10

### Added

- Something plain

[1.0.1]: https://google.com
[1.0.0]: https://google.com
"#;

    #[test]
    fn it_can_gather_link_defs() {
        let r = Changelog::try_from(A_CHANGELOG).unwrap();

        let links = gather_link_defs(r);

        assert_eq!(
            links.get("1.0.1").unwrap().to_owned(),
            "https://google.com".to_string()
        );
        assert_eq!(
            links.get("1.0.0").unwrap().to_owned(),
            "https://google.com".to_string()
        );
    }
}
