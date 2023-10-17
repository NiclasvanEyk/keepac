# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.9] - 2023-10-17

### Added

- `changelog path` as an alias of `changelog find`
- The column at which text wraps can be configured using the `KEEPAC_WRAP_AT` environment variable

### Changed

- The output now wraps after 85 characters

### Fixed

- `changelog release` omitting a few characters at the end of the printed summary
