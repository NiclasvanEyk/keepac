use std::collections::VecDeque;

use markdown::{mdast::Node, to_mdast, ParseOptions};

pub fn text_only(markdown: &str) -> String {
    let parsed = to_mdast(markdown, &ParseOptions::default()).unwrap();

    let mut children: VecDeque<&Node> = VecDeque::new();
    for node in parsed.children().unwrap() {
        children.push_back(node);
    }

    while let Some(child) = children.pop_front() {
        if let Some(grand_children) = child.children() {
            for grand_child in grand_children.into_iter().rev() {
                children.push_front(grand_child);
            }
        }

        match child {
            Node::Link(link) => {
                let title = &link.title;
                let text = title.clone().unwrap_or(String::from(""));
            }
            _ => {}
        };
    }

    return String::from(markdown);
}
