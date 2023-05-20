use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Changelog {
    pub title: String,
    pub releases: Releases,
}

#[derive(Debug)]
pub struct NextRelease {
    pub changes: Changes,
}

#[derive(Debug)]
pub struct Releases {
    pub next: Option<NextRelease>,
    pub past: Vec<Release>,
}

pub type ReleaseDate = String;

#[derive(Debug)]
pub struct Release {
    pub date: ReleaseDate, // Should be some data structure
    pub version: Version,
    pub changes: Changes,
    pub yanked: bool,
}

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub prerelease: bool,
    pub buildmetadata: Option<String>,
}

impl fmt::Display for Version {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn from_string(value: &str) -> Option<Self> {
        lazy_static! {
            static ref SEMVER: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
        }

        return SEMVER.captures(value).map(|captures| {
            return Version {
                major: captures.name("major").unwrap().as_str().parse().unwrap(),
                minor: captures.name("minor").unwrap().as_str().parse().unwrap(),
                patch: captures.name("patch").unwrap().as_str().parse().unwrap(),
                prerelease: captures.name("prerelease").is_some(),
                buildmetadata: captures
                    .name("buildmetadata")
                    .map(|meta| String::from(meta.as_str())),
            };
        });
    }
}

pub type ChangeDescription = String;

pub enum ChangeType {
    Added,
    Changed,
    Deprecated,
    Fixed,
    Removed,
    Security,
}

#[derive(Debug)]
pub struct Changes {
    pub added: Option<ChangeDescription>,
    pub changed: Option<ChangeDescription>,
    pub deprecated: Option<ChangeDescription>,
    pub fixed: Option<ChangeDescription>,
    pub removed: Option<ChangeDescription>,
    pub security: Option<ChangeDescription>,
}

impl Changes {
    pub fn empty() -> Self {
        return Self {
            added: Option::None,
            changed: Option::None,
            deprecated: Option::None,
            fixed: Option::None,
            removed: Option::None,
            security: Option::None,
        };
    }
}
