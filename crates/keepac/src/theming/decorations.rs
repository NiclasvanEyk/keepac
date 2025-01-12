/// All the various icons, box-drawing characters, and prefixes that are
/// decorational only.
pub struct Decorations {
    /// Character that gets repeated to draw a horizontal line.
    pub horizontal_rule: String,

    /// Whether to automatically insert a hr between each release.
    ///
    /// TODO: Maybe this is a user setting and not a theme option?
    pub horizontal_rule_between_releases: bool,

    /// The thing that gets written before the actual text of a list item.
    ///
    /// You don't need to include whitespace here.
    pub unordered_list_item: String,

    /// The thing that gets written before each line blockquote.
    ///
    /// You don't need to include whitespace here.
    pub blockquote_prefix: String,
}

impl Default for Decorations {
    fn default() -> Self {
        Decorations {
            horizontal_rule: '─'.into(),
            horizontal_rule_between_releases: true,
            unordered_list_item: "•".into(),
            blockquote_prefix: "┃".into(),
        }
    }
}
