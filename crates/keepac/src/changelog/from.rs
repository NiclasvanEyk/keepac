use crate::Changelog;

use tree_sitter::Parser;

#[derive(Debug)]
pub enum MarkdownParserError {
    FailedToLoadGrammar,
    CouldNotParseTree,
}

/// Get a Changelog from a string
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

// Get a Changelog from a file
// impl<'a> TryFrom<&'a str> for Changelog<'a> {
//     type Error = MarkdownParserError;
//
//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         let mut parser = Parser::new();
//
//         let Ok(_) = parser.set_language(&tree_sitter_md::LANGUAGE.into()) else {
//             return Err(MarkdownParserError::FailedToLoadGrammar);
//         };
//
//         let Some(tree) = parser.parse(value, None) else {
//             return Err(MarkdownParserError::CouldNotParseTree);
//         };
//
//         Ok(Self {
//             source: value,
//             tree,
//         })
//     }
// }

// Get a Changelog from a pathbuf that implements the recursive upwards iteration
