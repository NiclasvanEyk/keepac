use std::io::{Result, Write};
use termcolor::{Buffer, Color, ColorSpec, HyperlinkSpec, WriteColor};
use textwrap::{fill, Options as WrapOptions};

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

// struct RenderOptions {
//     width: usize,
//     padding_top: usize,
//     padding_left: usize,
// }

// impl Default for RenderOptions {
//     fn default() -> Self {
//         RenderOptions {
//             width: 80,
//             padding_top: 1,
//             padding_left: 2,
//         }
//     }
// }

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

enum RenderContext {
    VersionHeading,
    ChangeCategoryHeading,
}

enum ChangeCategory {
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

impl ChangeCategory {
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

fn horizontal_rule(width: usize, buffer: &mut Buffer) -> Result<()> {
    buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Ansi256(250)))
            .set_dimmed(true),
    )?;
    let rule = '┈'.to_string().repeat(width);
    write!(buffer, "{}", rule)?;
    buffer.reset()
}

pub fn render(source_buffer: &mut Buffer, source: &str) -> Result<()> {
    let parser = Parser::new_ext(source, Options::all());
    let mut render_context: Option<RenderContext> = None;
    let mut inside_link = false;

    let width = 80;
    let mut buffer = source_buffer.clone();
    write!(source_buffer, "\n")?;

    let mut colors = ColorSpec::new();
    buffer.set_color(&colors)?;

    for (event, range) in parser.into_offset_iter() {
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
                        horizontal_rule(width, source_buffer)?;
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
                    inside_link = true;
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
                        &WrapOptions::new(width)
                            .initial_indent("  ")
                            .subsequent_indent("  "),
                        "\n\n",
                    )?
                }
                TagEnd::Heading(_) => {
                    render_context = None;
                    buffer.reset()?;
                    flush_to_source(
                        &mut buffer,
                        source_buffer,
                        &WrapOptions::new(width)
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
                        &WrapOptions::new(width)
                            .initial_indent("  - ")
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
                    buffer.set_color(&colors)?;
                    inside_link = false;
                }
                TagEnd::Image => {}
                TagEnd::MetadataBlock(_) => {}
            },
            Event::Text(text) => match render_context {
                None => write!(buffer, "{}", text)?,
                Some(ref context) => match context {
                    RenderContext::VersionHeading => {
                        if text.starts_with(" - ") {
                            // Release date
                            let date = text.strip_prefix(" - ");
                            match date {
                                Some(date) => write!(buffer, " - \u{eab0} {}", date)?,
                                // Fallback
                                None => write!(buffer, "{}", text)?,
                            }
                        } else {
                            // Version tag
                            write!(buffer, "\u{ea66} {}", text)?
                        }
                    }
                    RenderContext::ChangeCategoryHeading => {
                        let category = ChangeCategory::parse(text.as_ref());
                        let icon = match &category {
                            Some(c) => match c {
                                ChangeCategory::Added => Some("\u{eadc}"),
                                ChangeCategory::Changed => Some("\u{eae0}"),
                                ChangeCategory::Deprecated => Some("\u{ea98}"),
                                ChangeCategory::Removed => Some("\u{eadf}"),
                                ChangeCategory::Fixed => Some("\u{ead8}"),
                                ChangeCategory::Security => Some("\u{eb53}"),
                            },
                            None => None,
                        };

                        let colors_before = colors.clone();

                        match &category {
                            Some(c) => match c {
                                ChangeCategory::Added => {
                                    colors.set_fg(Some(Color::Green));
                                }
                                ChangeCategory::Changed => {
                                    colors.set_fg(Some(Color::Yellow));
                                }
                                ChangeCategory::Deprecated => {
                                    colors.set_fg(Some(Color::Yellow));
                                }
                                ChangeCategory::Removed => {
                                    colors.set_fg(Some(Color::Red));
                                }
                                ChangeCategory::Fixed => {
                                    colors.set_fg(Some(Color::Cyan));
                                }
                                ChangeCategory::Security => {
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

#[cfg(test)]
mod tests {
    use termcolor::{BufferWriter, ColorChoice};

    use super::*;

    /// Renders the source and compares it to the expected result.
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
            "\u{1b}]8;;Google\u{1b}\\https://google.com\u{1b}]8;;\u{1b}\\",
        );
    }

    #[test]
    pub fn single_word() {
        assert_renders_plain("word", "  \n  word\n  \n  ");
    }

    #[test]
    pub fn lists() {
        assert_renders_plain("- This is a really long list item, that spans multiple lines and should be wrapped correctly. It even has punctuation!", "");
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
            "  This is one paragraph. This is is another paragraph. The sentences should not\n  be organized by line, but grouped together inside a paragraph.\n  \n  ",
        );
    }
}
