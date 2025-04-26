use termcolor::ColorSpec;

pub struct Colors {
    /// The colors used for rendering text, e.g. in plain paragraphs.
    pub defaults: ColorSpec,

    /// The default colors for various headings.
    pub headings: HeadingColors,

    /// Defines the colors for several "border-like" characters.
    ///
    /// Right now that list consists of:
    /// - blockquote borders / prefix
    /// - unordered list item prefix
    /// - horizontal rule
    pub borders: ColorSpec,
}

impl Default for Colors {
    fn default() -> Self {
        // Terminals are just dark by default, so we'll follow suit
        detect_colors().unwrap_or(dark_colors())
    }
}

/// The default colors for various headings.
pub struct HeadingColors {
    h1: ColorSpec,
    h2: ColorSpec,
    h3: ColorSpec,
    h4: ColorSpec,
    h5: ColorSpec,
    h6: ColorSpec,
    // TODO: Incorporate overrides for specific types of changes. Right now this
    //       is implemented as `h3`s.
}

fn dark_colors() -> Colors {
    todo!()
}

fn light_colors() -> Colors {
    todo!()
}

fn detect_colors() -> Option<Colors> {
    todo!()
}
