use std::fmt::Display;

use tree_sitter::Node;

use crate::Changelog;

struct ChangelogRenderer<'a, 'f> {
    changelog: &'a Changelog<'a>,
    f: &'a mut std::fmt::Formatter<'f>,
    // Maps the [1.0.0] to their actual URLs at the bottom
    // link_defs: HashMap<&'a str, &'a str>,
}

impl<'a, 'f> ChangelogRenderer<'a, 'f> {
    pub fn render(&mut self) -> std::fmt::Result {
        let mut cursor = self.changelog.tree.walk();

        // The first node is always the document one
        cursor.goto_first_child();
        cursor.goto_first_child();
        cursor.goto_next_sibling();
        cursor.goto_next_sibling();

        let node = cursor.node();
        writeln!(self.f, "NODE: {}", node)?;

        let contents = node.child_by_field_name("heading_content");
        match contents {
            Some(c) => self.render_heading(c),
            None => {}
        };

        Ok(())
    }

    fn render_heading(&self, node: Node) {
        let res = node.utf8_text(self.changelog.source.as_bytes()).unwrap();
        println!("HEADING: {}", res);
    }

    fn render_inline(&self, node: Node) {}
}

impl<'a> Display for Changelog<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let link_defs = gather_link_defs(&self.tree, self.source);
        let mut renderer = ChangelogRenderer {
            changelog: self,
            f,
            // link_defs,
        };
        renderer.render()
    }
}

pub fn render(changelog: &str) {
    let renderer = Changelog::try_from(changelog).unwrap();
    println!("{}", renderer);
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
