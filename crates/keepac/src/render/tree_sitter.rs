use std::{
    fmt::Formatter,
    io::{Result, Write},
};

use tree_sitter::Node;

#[derive(Debug, Default)]
pub(crate) struct TreeSitterTerminalHighlighter {}

impl TreeSitterTerminalHighlighter {
    pub fn render_heading(&self, node: &Node, formatter: &mut Formatter) -> Result<()> {
        Ok(())
    }

    pub fn render_inline_link(
        &self,
        node: &Node,
        source: &[u8],
        formatter: &mut impl Write,
    ) -> Result<()> {
        let link_text = node.child(0).unwrap().utf8_text(source).unwrap();
        let link_address = node.child(1).unwrap().utf8_text(source).unwrap();

        write!(
            formatter,
            "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
            link_text, link_address
        )
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Query;

    use crate::parse::parse_markdown;

    use super::*;

    #[test]
    pub fn it_renders_links() {
        let highlighter = TreeSitterTerminalHighlighter::default();
        let source = "[Google](https://google.com)";
        let tree = parse_markdown(source).unwrap();
        println!("{:?}", tree);

        let root_node = tree.root_node();
        let section = root_node.child(0).unwrap();
        println!("{}", section.grammar_name());
        let paragraph = section.child(0).unwrap();
        let inline = paragraph.child(0).unwrap();

        // let foo = markdown_query("(inline_link)");

        println!("{:?}", inline);
        let mut buffer = Vec::new();

        highlighter
            .render_inline_link(&inline, source.as_bytes(), &mut buffer)
            .unwrap();
        let output = std::str::from_utf8(buffer.as_slice()).unwrap();

        assert_eq!(
            output,
            "\u{1b}]8;;Google\u{1b}\\https://google.com\u{1b}]8;;\u{1b}\\"
        );
    }
}
