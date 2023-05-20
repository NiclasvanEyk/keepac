use lazy_static::lazy_static;
use regex::Regex;
use std::vec;

use crate::parse::structs::{
    Changelog, Changes, NextRelease, Release, ReleaseDate, Releases, Version,
};

struct PendingRelease {
    pub heading: String,
    pub changes: Changes,
    // pub current_change_type: Option<String>,
}

impl PendingRelease {
    pub fn new(heading: &str) -> Self {
        return PendingRelease {
            heading: String::from(heading),
            changes: Changes::empty(),
            // current_change_type: Option::None,
        };
    }

    pub fn finalized(self) -> Result<Release, String> {
        let (version, release_date) = parse_release_heading(self.heading.as_str())?;

        return Result::Ok(Release {
            version,
            release_date,
            changes: self.changes,
            yanked: false, // TODO
        });
    }
}

pub fn parse(source: &str) -> Result<Changelog, String> {
    let mut lines = source.lines().map(|line| line.trim());

    let mut heading = "";
    for line in lines.by_ref() {
        if is_title_heading(line) {
            heading = extract_title_heading(line);
            break;
        }
    }

    if heading.trim().is_empty() {
        return Result::Err(String::from(
            "No or empty heading found! Ensure there is exactly one level 1 heading, e.g. '# My Heading'!",
        ));
    }

    let mut next_release = Option::None;
    let mut check_for_next_release = true;
    let mut pending_release_is_next_release = false;

    let mut releases: Vec<Release> = vec![];
    let mut pending_release: Option<PendingRelease> = Option::None;

    // The first release could be the unreleased one!
    for line in lines.by_ref() {
        if check_for_next_release && is_next_release_heading(line) {
            check_for_next_release = false;
            pending_release_is_next_release = true;
            continue;
        }

        if is_release_heading(line) {
            // We found a release heading!
            // First we need to close the existing release section, if there is any
            if pending_release.is_some() {
                if pending_release_is_next_release {
                    next_release = Option::Some(NextRelease {
                        changes: pending_release.unwrap().changes,
                    });
                    pending_release_is_next_release = false;
                } else {
                    releases.push(pending_release.unwrap().finalized()?);
                }
            }

            let heading = extract_release_heading(line);
            pending_release = Option::Some(PendingRelease::new(heading));

            check_for_next_release = false;
            continue;
        }

        if pending_release.is_some() {
            // TODO
            // current_release_contents.push(line);
        }
    }

    // Flush contents of current release!
    if pending_release.is_some() {
        if pending_release_is_next_release {
            next_release = Option::Some(NextRelease {
                changes: pending_release.unwrap().changes,
            });
        } else {
            releases.push(pending_release.unwrap().finalized()?);
        }
    }

    return Result::Ok(Changelog {
        title: String::from(heading),
        releases: Releases {
            next: next_release,
            past: releases,
        },
    });
}

fn is_title_heading(line: &str) -> bool {
    return line.starts_with("# ");
}

fn extract_title_heading(line: &str) -> &str {
    return &line[2..];
}

fn is_release_heading(line: &str) -> bool {
    return line.starts_with("## ");
}

fn is_next_release_heading(line: &str) -> bool {
    return line.starts_with("## [Unreleased]");
}

fn extract_release_heading(line: &str) -> &str {
    return &line[3..];
}

fn parse_release_heading(heading_content: &str) -> Result<(Version, ReleaseDate), String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^\[(.*)\] - ([0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9])$").unwrap();
    }

    let parse_result = RE.captures(heading_content);
    if parse_result.is_none() {
        return Result::Err(String::from(format!(
            "Not a valid release heading: '{}'!",
            heading_content
        )));
    }

    let captures = parse_result.unwrap();

    let version_string = captures.get(1).unwrap().as_str();
    let version = Version::from_string(version_string);
    if version.is_none() {
        return Result::Err(String::from(format!(
            "'{}' is not a valid semantic version!",
            version_string
        )));
    }

    let date = captures.get(2).unwrap().as_str();

    return Result::Ok((version.unwrap(), String::from(date)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_the_keepachangelog_example() {
        let parse_result = parse(
            r#"
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1] - 2023-03-05

### Added

- Arabic translation (#444).
- v1.1 French translation.
- v1.1 Dutch translation (#371).
- v1.1 Russian translation (#410).
- v1.1 Japanese translation (#363).
- v1.1 Norwegian Bokmål translation (#383).
- v1.1 "Inconsistent Changes" Turkish translation (#347).
- Default to most recent versions available for each languages
- Display count of available translations (26 to date!)
- Centralize all links into `/data/links.json` so they can be updated easily

### Fixed

- Improve French translation (#377).
- Improve id-ID translation (#416).
- Improve Persian translation (#457).
- Improve Russian translation (#408).
- Improve Swedish title (#419).
- Improve zh-CN translation (#359).
- Improve French translation (#357).
- Improve zh-TW translation (#360, #355).
- Improve Spanish (es-ES) transltion (#362).
- Foldout menu in Dutch translation (#371).
- Missing periods at the end of each change (#451).
- Fix missing logo in 1.1 pages
- Display notice when translation isn't for most recent version
- Various broken links, page versions, and indentations.

### Changed

- Upgrade dependencies: Ruby 3.2.1, Middleman, etc.

### Removed

- Unused normalize.css file
- Identical links assigned in each translation file
- Duplicate index file for the english version

## [1.1.0] - 2019-02-15

### Added

- Danish translation (#297).
- Georgian translation from (#337).
- Changelog inconsistency section in Bad Practices.

### Fixed

- Italian translation (#332).
- Indonesian translation (#336).

## [1.0.0] - 2017-06-20

### Added

- New visual identity by [@tylerfortune8](https://github.com/tylerfortune8).
- Version navigation.
- Links to latest released version in previous versions.
- "Why keep a changelog?" section.
- "Who needs a changelog?" section.
- "How do I make a changelog?" section.
- "Frequently Asked Questions" section.
- New "Guiding Principles" sub-section to "How do I make a changelog?".
- Simplified and Traditional Chinese translations from [@tianshuo](https://github.com/tianshuo).
- German translation from [@mpbzh](https://github.com/mpbzh) & [@Art4](https://github.com/Art4).
- Italian translation from [@azkidenz](https://github.com/azkidenz).
- Swedish translation from [@magol](https://github.com/magol).
- Turkish translation from [@emreerkan](https://github.com/emreerkan).
- French translation from [@zapashcanon](https://github.com/zapashcanon).
- Brazilian Portuguese translation from [@Webysther](https://github.com/Webysther).
- Polish translation from [@amielucha](https://github.com/amielucha) & [@m-aciek](https://github.com/m-aciek).
- Russian translation from [@aishek](https://github.com/aishek).
- Czech translation from [@h4vry](https://github.com/h4vry).
- Slovak translation from [@jkostolansky](https://github.com/jkostolansky).
- Korean translation from [@pierceh89](https://github.com/pierceh89).
- Croatian translation from [@porx](https://github.com/porx).
- Persian translation from [@Hameds](https://github.com/Hameds).
- Ukrainian translation from [@osadchyi-s](https://github.com/osadchyi-s).

### Changed

- Start using "changelog" over "change log" since it's the common usage.
- Start versioning based on the current English version at 0.3.0 to help
  translation authors keep things up-to-date.
- Rewrite "What makes unicorns cry?" section.
- Rewrite "Ignoring Deprecations" sub-section to clarify the ideal
  scenario.
- Improve "Commit log diffs" sub-section to further argument against
  them.
- Merge "Why can’t people just use a git log diff?" with "Commit log
  diffs".
- Fix typos in Simplified Chinese and Traditional Chinese translations.
- Fix typos in Brazilian Portuguese translation.
- Fix typos in Turkish translation.
- Fix typos in Czech translation.
- Fix typos in Swedish translation.
- Improve phrasing in French translation.
- Fix phrasing and spelling in German translation.

### Removed

- Section about "changelog" vs "CHANGELOG".

## [0.3.0] - 2015-12-03

### Added

- RU translation from [@aishek](https://github.com/aishek).
- pt-BR translation from [@tallesl](https://github.com/tallesl).
- es-ES translation from [@ZeliosAriex](https://github.com/ZeliosAriex).

## [0.2.0] - 2015-10-06

### Changed

- Remove exclusionary mentions of "open source" since this project can
  benefit both "open" and "closed" source projects equally.

## [0.1.0] - 2015-10-06

### Added

- Answer "Should you ever rewrite a change log?".

### Changed

- Improve argument against commit logs.
- Start following [SemVer](https://semver.org) properly.

## [0.0.8] - 2015-02-17

### Changed

- Update year to match in every README example.
- Reluctantly stop making fun of Brits only, since most of the world
  writes dates in a strange way.

### Fixed

- Fix typos in recent README changes.
- Update outdated unreleased diff link.

## [0.0.7] - 2015-02-16

### Added

- Link, and make it obvious that date format is ISO 8601.

### Changed

- Clarified the section on "Is there a standard change log format?".

### Fixed

- Fix Markdown links to tag comparison URL with footnote-style links.

## [0.0.6] - 2014-12-12

### Added

- README section on "yanked" releases.

## [0.0.5] - 2014-08-09

### Added

- Markdown links to version tags on release headings.
- Unreleased section to gather unreleased changes and encourage note
  keeping prior to releases.

## [0.0.4] - 2014-08-09

### Added

- Better explanation of the difference between the file ("CHANGELOG")
  and its function "the change log".

### Changed

- Refer to a "change log" instead of a "CHANGELOG" throughout the site
  to differentiate between the file and the purpose of the file — the
  logging of changes.

### Removed

- Remove empty sections from CHANGELOG, they occupy too much space and
  create too much noise in the file. People will have to assume that the
  missing sections were intentionally left out because they contained no
  notable changes.

## [0.0.3] - 2014-08-09

### Added

- "Why should I care?" section mentioning The Changelog podcast.

## [0.0.2] - 2014-07-10

### Added

- Explanation of the recommended reverse chronological release ordering.

## [0.0.1] - 2014-05-31

### Added

- This CHANGELOG file to hopefully serve as an evolving example of a
  standardized open source project CHANGELOG.
- CNAME file to enable GitHub Pages custom domain.
- README now contains answers to common questions about CHANGELOGs.
- Good examples and basic guidelines, including proper date formatting.
- Counter-examples: "What makes unicorns cry?".

[unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.3.0...v1.0.0
[0.3.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.8...v0.1.0
[0.0.8]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.0.1
"#,
        );

        if parse_result.is_err() {
            println!("{}", parse_result.err().unwrap());
            assert!(false);
            return;
        }

        let changelog = parse_result.unwrap();

        assert_eq!("Changelog", changelog.title);

        print!("{:?}", changelog);
    }
}
