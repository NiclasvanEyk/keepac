/// The main entry point for manipulating things with the changelog / the "public" API.
pub mod changelog;
/// Find CHANGELOG.md files on the filesystem tree.
pub mod find;
/// Parse changelogs given a few structural constraints.
pub mod parse;
/// Render out changelogs to the terminal.
pub mod render;

/// Treesitter utilities for markdown files
pub mod markdown;

pub use changelog::Changelog;
