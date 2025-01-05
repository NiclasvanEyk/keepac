use crate::Changelog;
use std::collections::HashMap;
use streaming_iterator::StreamingIterator;
use thiserror::Error;
use tree_sitter::{LanguageError, Parser, Tree};

#[derive(Error, Debug)]
pub enum MarkdownParserError {
    #[error("Failed to load markdown grammar")]
    FailedToLoadGrammar(#[from] LanguageError),

    #[error("Failed to parse the changelog markdown file")]
    CouldNotParseTree,
}

pub(crate) fn parse_markdown(value: impl AsRef<[u8]>) -> Result<Tree, MarkdownParserError> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_md::LANGUAGE.into())?;

    let Some(tree) = parser.parse(value, None) else {
        return Err(MarkdownParserError::CouldNotParseTree);
    };

    Ok(tree)
}

#[derive(Debug)]
pub struct Version<'a> {
    pub name: &'a str,
    pub released_at: Option<&'a str>,
    pub href: Option<&'a str>,
}

pub fn parse_versions<'c>(changelog: &'c Changelog<'c>) -> Vec<Version<'c>> {
    changelog
        .document
        .query_cursor("(atx_heading (atx_h2_marker) heading_content: (inline) @content)")
        .unwrap()
        .captured_nodes()
        .iter()
        .map(|heading_node| {
            let parts: Vec<&str> = heading_node
                .utf8_text(changelog.document.source.as_ref())
                .unwrap()
                .split(" - ")
                .collect();

            Version {
                name: parts[0],
                released_at: parts.get(1).copied(),
                href: None,
            }
        })
        .collect()
}

pub fn gather_link_defs<'a>(changelog: &'a Changelog<'a>) -> HashMap<&'a str, &'a str> {
    let mut cursor = changelog
        .document
        .query_cursor(
            "(link_reference_definition (link_label) @label (link_destination) @destination)",
        )
        .unwrap();
    let mut matches = cursor.matches();

    let mut links = HashMap::new();
    while let Some(m) = matches.next() {
        let captures = m.captures;

        let label = captures
            .first()
            .unwrap()
            .node
            .utf8_text(changelog.document.source.as_ref())
            .unwrap()
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap();
        let destination = captures
            .get(1)
            .unwrap()
            .node
            .utf8_text(changelog.document.source.as_ref())
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

        let links = gather_link_defs(&r);

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
