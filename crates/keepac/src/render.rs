mod tree_sitter;

use std::fmt::Display;

use crate::Changelog;

// #[derive(Default, Debug)]
// pub struct ChangelogRenderContext {}
//
// #[derive(Default, Debug)]
// pub struct TerminalChangelogRenderer {
//     context: ChangelogRenderContext,
// }
//
// impl TerminalChangelogRenderer {
//     pub fn render(source: &str, formatter: &mut std::fmt::Formatter) -> std::io::Result<()> {
//         Ok(())
//     }
// }

// struct ChangelogRenderer<'a, 'f> {
//     changelog: &'a Changelog<'a>,
//     f: &'a mut std::fmt::Formatter<'f>,
//     // Maps the [1.0.0] to their actual URLs at the bottom
//     // link_defs: HashMap<&'a str, &'a str>,
// }

// impl<'a, 'f> ChangelogRenderer<'a, 'f> {
//     pub fn render(&mut self) -> std::fmt::Result {
//         let mut cursor = self.changelog.tree.walk();
//
//         // The first node is always the document one
//         cursor.goto_first_child();
//         cursor.goto_first_child();
//         cursor.goto_next_sibling();
//         cursor.goto_next_sibling();
//
//         let node = cursor.node();
//         writeln!(self.f, "NODE: {}", node)?;
//
//         let contents = node.child_by_field_name("heading_content");
//         match contents {
//             Some(c) => self.render_heading(c),
//             None => {}
//         };
//
//         Ok(())
//     }
//
//     fn render_heading(&self, node: Node) {
//         let res = node.utf8_text(self.changelog.source.as_bytes()).unwrap();
//         println!("HEADING: {}", res);
//     }
//
//     fn render_inline(&self, node: Node) {}
// }

impl<'a> Display for Changelog<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(self.document.source.as_ref()).unwrap()
        )
    }
}

pub fn render(changelog: &str) {
    println!("{}", Changelog::try_from(changelog).unwrap());
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
    fn it_renders() {
        render(A_CHANGELOG)
    }
}
