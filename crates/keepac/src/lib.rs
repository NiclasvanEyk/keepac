/// The main entry point for manipulating things with the changelog / the "public" API.
pub mod changelog;
/// Find CHANGELOG.md files on the filesystem tree.
pub mod find;
/// Parse changelogs given a few structural constraints.
pub mod parse;

/// Treesitter utilities for markdown files
pub mod markdown;

pub mod highlight;

pub mod theme;

pub use changelog::Changelog;
