use std::io::{Result, Write};

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use termcolor::{Buffer, Color, ColorSpec, HyperlinkSpec, WriteColor};
use textwrap::{fill, Options as WrapOptions};

use crate::theming;

fn flush_to_source(
    buffer: &mut Buffer,
    source_buffer: &mut Buffer,
    wrap_options: &WrapOptions,
    flush_str: &str,
) -> Result<()> {
    let str = std::str::from_utf8(buffer.as_slice()).unwrap();
    let wrapped = fill(str, wrap_options);

    source_buffer.write_all(wrapped.as_bytes())?;
    source_buffer.write_all(flush_str.as_bytes())?;

    source_buffer.flush()?;
    buffer.clear();

    Ok(())
}

/// What are we rendering right now.
/// Imporant for e.g. text styling, since we e.g. want to e.g. highlight
enum RenderContext {
    VersionHeading,
    ChangeCategoryHeading,
}

enum BlockComponent {
    Heading,
    BlockQuote,
    ListItem, // Maybe also just List?
}

enum InlineComponent {
    /// You can have a bold/code/italic link
    Link,
}

/// TODO
///
/// This is basically enabling us to e.g. JUST temporarily force the text being
/// bold, and then return to the previous state without also modifying something
/// else. We can not simply set bold to false inside bold, and then for the
/// end tag set it to `false` again, since e.g. somebody may have used the bold
/// text in a heading, which we might want to always print bold. Or italic.
enum InlineModifier {
    Bold,
    Italic,
    Code,
}

/// What we are rendering here
///
/// This could probably also be a stack-like structure, but this way it is maybe
/// a bit easier to see the possible combinations.
struct Context {
    block_component: Option<RenderContext>,
    inline_component: Option<InlineComponent>,
    inline_modifier: Option<InlineModifier>,
}

/// A category for the changes inside a release.
///
/// Currently these strictly follow the Keep a Changelog ones, but in theory we
/// could easily extend this to dynamic strings (based on some kind of
/// configuration or theming), and be a bit more flexible. Maybe these could
/// also be inferred from the existing changelog text, but that would likely
/// require multiple passes not a fan of that.
enum ChangeType {
    /// New features.
    Added,

    /// Changes in existing functionality.
    Changed,

    /// Soon-to-be removed features.
    Deprecated,

    /// Now removed features.
    Removed,

    /// Any bug fixes.
    Fixed,

    /// In case of vulnerabilities.
    Security,
}

impl ChangeType {
    fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "added" => Some(Self::Added),
            "changed" => Some(Self::Changed),
            "deprecated" => Some(Self::Deprecated),
            "removed" => Some(Self::Removed),
            "fixed" => Some(Self::Fixed),
            "security" => Some(Self::Security),
            _ => None,
        }
    }
}

fn horizontal_rule(buffer: &mut Buffer, width: usize, color: Color, character: &str) -> Result<()> {
    buffer.set_color(ColorSpec::new().set_fg(Some(color)).set_dimmed(true))?;
    let rule = character.repeat(width);
    write!(buffer, "{}", rule)?;
    buffer.reset()
}

// TODO: Refactor styling. Pre-compute colors upfront per "component", and have
//       modifiers(?) for stuff like emph, link, or inline code.

pub struct RenderingOptions {
    /// The max width of a line of charactes.
    pub width: usize,

    /// Icons used throughout several components.
    pub icons: theming::Icons,

    /// All the various icons, box-drawing characters, and prefixes that are
    /// decorational only.
    pub decorations: theming::Decorations,

    pub colors: theming::Colors,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        RenderingOptions {
            width: 80,
            icons: theming::Icons::nerdfont(),
            decorations: theming::Decorations::default(),
            colors: theming::Colors::default(),
        }
    }
}

pub fn render_with_options(
    source_buffer: &mut Buffer,
    source: &str,
    options: &RenderingOptions,
) -> Result<()> {
    let parser = Parser::new_ext(source, Options::all());
    let mut render_context: Option<RenderContext> = None;

    let mut buffer = source_buffer.clone();
    write!(source_buffer, "\n")?;

    let mut colors = ColorSpec::new();
    buffer.set_color(&colors)?;

    for (event, _) in parser.into_offset_iter() {
        // eprintln!("{:?} {:?}", range, event);

        match event {
            Event::Start(start) => match start {
                Tag::Paragraph => {}
                Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                } => match level {
                    HeadingLevel::H1 => {
                        colors.set_fg(Some(Color::Blue));
                        colors.set_bold(true);
                        buffer.set_color(&colors)?;
                    }
                    HeadingLevel::H2 => {
                        horizontal_rule(
                            source_buffer,
                            options.width,
                            Color::Cyan,
                            &options.decorations.horizontal_rule,
                        )?;
                        write!(source_buffer, "\n\n")?;
                        render_context = Some(RenderContext::VersionHeading);
                        colors.set_bold(false);
                        colors.set_fg(Some(Color::Cyan));
                        buffer.set_color(&colors)?;
                    }
                    HeadingLevel::H3 => {
                        render_context = Some(RenderContext::ChangeCategoryHeading);
                        colors.set_bold(false);
                        colors.set_fg(Some(Color::Green));
                        buffer.set_color(&colors)?;
                    }
                    HeadingLevel::H4 => {
                        colors.set_fg(Some(Color::Magenta));
                        buffer.set_color(&colors)?;
                    }
                    HeadingLevel::H5 => {
                        colors.set_fg(Some(Color::Red));
                        buffer.set_color(&colors)?;
                    }
                    HeadingLevel::H6 => {
                        colors.set_fg(Some(Color::Red));
                        buffer.set_color(&colors)?;
                    }
                },
                Tag::BlockQuote(_) => {}
                Tag::CodeBlock(_) => {}
                Tag::HtmlBlock => {}
                Tag::List(_) => {}
                Tag::Item => {}
                Tag::FootnoteDefinition(_) => {}
                Tag::DefinitionList => {}
                Tag::DefinitionListTitle => {}
                Tag::DefinitionListDefinition => {}
                Tag::Table(_) => {}
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Emphasis => {}
                Tag::Strong => {}
                Tag::Strikethrough => {}
                Tag::Link {
                    link_type: _,
                    dest_url,
                    title: _,
                    id: _,
                } => {
                    buffer.set_hyperlink(&HyperlinkSpec::open(dest_url.as_bytes()))?;
                    colors.set_underline(true);
                    colors.set_fg(Some(Color::Cyan));
                    buffer.set_color(&colors)?;
                }
                Tag::Image {
                    link_type: _,
                    dest_url: _,
                    title: _,
                    id: _,
                } => {}
                Tag::MetadataBlock(_) => {}
            },
            Event::End(end) => match end {
                TagEnd::Paragraph => {
                    buffer.reset()?;
                    flush_to_source(
                        &mut buffer,
                        source_buffer,
                        &WrapOptions::new(options.width)
                            .initial_indent("  ")
                            .subsequent_indent("  "),
                        "\n\n",
                    )?
                }
                TagEnd::Heading(_) => {
                    render_context = None;
                    colors.set_bold(false);
                    buffer.set_color(&colors)?;
                    buffer.reset()?;
                    flush_to_source(
                        &mut buffer,
                        source_buffer,
                        &WrapOptions::new(options.width)
                            .initial_indent("  ")
                            .subsequent_indent("  "),
                        "\n\n",
                    )?;
                }
                TagEnd::BlockQuote(_) => {}
                TagEnd::CodeBlock => {}
                TagEnd::HtmlBlock => {}
                TagEnd::List(_) => {
                    buffer.reset()?;
                    write!(source_buffer, "\n")?;
                }
                TagEnd::Item => {
                    buffer.reset()?;
                    flush_to_source(
                        &mut buffer,
                        source_buffer,
                        &WrapOptions::new(options.width)
                            .initial_indent(&format!(
                                "  {} ",
                                options.decorations.unordered_list_item
                            ))
                            .subsequent_indent("    "),
                        "\n",
                    )?;
                }
                TagEnd::FootnoteDefinition => {}
                TagEnd::DefinitionList => {}
                TagEnd::DefinitionListTitle => {}
                TagEnd::DefinitionListDefinition => {}
                TagEnd::Table => {}
                TagEnd::TableHead => {}
                TagEnd::TableRow => {}
                TagEnd::TableCell => {}
                TagEnd::Emphasis => {}
                TagEnd::Strong => {}
                TagEnd::Strikethrough => {}
                TagEnd::Link => {
                    buffer.set_hyperlink(&HyperlinkSpec::close())?;
                    colors.set_underline(false);
                    colors.set_fg(None);
                    colors.reset();
                    buffer.set_color(&colors)?;
                }
                TagEnd::Image => {}
                TagEnd::MetadataBlock(_) => {}
            },
            Event::Text(text) => match render_context {
                None => write!(buffer, "{}", text)?,
                Some(ref context) => match context {
                    RenderContext::VersionHeading => {
                        // TODO: Maybe somehow put these in a separate buffer
                        // so that we can left align the version and right-align
                        // the date. That would be _really_ cool!
                        // Maybe we could even merge the first subheading and
                        // the version/date part into one? Would be a bit
                        // against the document structure, but that does not
                        // really matter that much.
                        //
                        // Alternatively, we could render the subheadings as
                        // HRs like this:
                        // TAG -----------------------------------------Date
                        if text.starts_with(" - ") {
                            // Release date
                            let date = text.strip_prefix(" - ");
                            match date {
                                Some(date) => {
                                    write!(buffer, " - {} {}", options.icons.calendar, date)?
                                }
                                // Fallback
                                None => write!(buffer, "{}", text)?,
                            }
                        } else {
                            // Version tag
                            write!(buffer, "{} {}", options.icons.tag, text)?
                        }
                    }
                    RenderContext::ChangeCategoryHeading => {
                        let category = ChangeType::parse(text.as_ref());
                        let icon = match &category {
                            Some(c) => match c {
                                ChangeType::Added => Some(options.icons.added),
                                ChangeType::Changed => Some(options.icons.changed),
                                ChangeType::Deprecated => Some(options.icons.deprecated),
                                ChangeType::Removed => Some(options.icons.removed),
                                ChangeType::Fixed => Some(options.icons.fixed),
                                ChangeType::Security => Some(options.icons.security),
                            },
                            None => None,
                        };

                        let colors_before = colors.clone();

                        match &category {
                            Some(c) => match c {
                                ChangeType::Added => {
                                    colors.set_fg(Some(Color::Green));
                                }
                                ChangeType::Changed => {
                                    colors.set_fg(Some(Color::Yellow));
                                }
                                ChangeType::Deprecated => {
                                    colors.set_fg(Some(Color::Yellow));
                                }
                                ChangeType::Removed => {
                                    colors.set_fg(Some(Color::Red));
                                }
                                ChangeType::Fixed => {
                                    colors.set_fg(Some(Color::Cyan));
                                }
                                ChangeType::Security => {
                                    colors.set_fg(Some(Color::Blue));
                                }
                            },
                            None => {}
                        };

                        buffer.set_color(&colors)?;
                        match icon {
                            Some(icon) => write!(buffer, "{} {}", icon, text)?,
                            None => write!(buffer, "{}", text)?,
                        };

                        colors = colors_before;
                    }
                },
            },
            Event::Code(code) => write!(buffer, "{}", code)?,
            Event::InlineMath(_) => {}
            Event::DisplayMath(_) => {}
            Event::Html(_) => {}
            Event::InlineHtml(_) => {}
            Event::FootnoteReference(_) => {}
            Event::SoftBreak => write!(buffer, " ")?,
            Event::HardBreak => {}
            Event::Rule => write!(buffer, "\n----------\n")?,
            Event::TaskListMarker(checked) => match checked {
                true => write!(buffer, "☑︎")?,
                false => write!(buffer, "☐")?,
            },
        }
    }

    Ok(())
}

pub fn render(source_buffer: &mut Buffer, source: &str) -> Result<()> {
    render_with_options(source_buffer, source, &RenderingOptions::default())
}

#[cfg(test)]
mod tests {
    use termcolor::{BufferWriter, ColorChoice};

    use super::*;

    /// Renders the source and compares it to the expected result.
    ///
    /// This version actually renders out ANSI escape sequences etc.
    fn assert_renders(source: &str, expected_result: &str) {
        let writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = writer.buffer();

        render(&mut buffer, source).unwrap();

        let rendered = std::str::from_utf8(buffer.as_slice()).unwrap();

        assert_eq!(rendered, expected_result);
    }

    /// Renders the source and compares it to the expected result.
    fn assert_renders_plain(source: &str, expected_result: &str) {
        let writer = BufferWriter::stdout(ColorChoice::Never);
        let mut buffer = writer.buffer();

        render(&mut buffer, source).unwrap();

        let rendered = std::str::from_utf8(buffer.as_slice()).unwrap();

        assert_eq!(rendered, expected_result);
    }

    #[test]
    pub fn it_renders_links() {
        assert_renders(
            "[Google](https://google.com)",
            "\n  \u{1b}[0m\u{1b}]8;;https://google.com\u{1b}\\\u{1b}[0m\u{1b}[4m\u{1b}[36mGoogle\u{1b}]8;;\u{1b}\\\u{1b}[0m\u{1b}[36m\u{1b}[0m\n\n",
        );
    }

    #[test]
    pub fn it_renders_links_plus_surrounding_text() {
        assert_renders(
            "The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).",
            "",
        )
    }

    #[test]
    pub fn single_word() {
        assert_renders_plain("word", "\n  word\n\n");
    }

    #[test]
    pub fn lists() {
        assert_renders_plain(
            "- This is a really long list item, that spans multiple lines and should be wrapped correctly. It even has punctuation!",
            "\n  • This is a really long list item, that spans multiple lines and should be\n    wrapped correctly. It even has punctuation!\n\n"
        );
    }

    #[test]
    pub fn it_correctly_wraps_text() {
        assert_renders_plain(
            r#"
This is one paragraph.
This is is another paragraph.
The sentences should not be organized by line, but grouped together inside a paragraph.
"#
            .trim(),
            "\n  This is one paragraph. This is is another paragraph. The sentences should not\n  be organized by line, but grouped together inside a paragraph.\n\n",
        );
    }
}
