# `keepac`

> An opiniated way of managing changelogs adhering to the [keepachangelog](https://keepachangelog.com/) guidelines.

![](https://niclasvaneyk.github.io/keepac/demo.light.gif#gh-light-mode-only)
![](https://niclasvaneyk.github.io/keepac/demo.dark.gif#gh-dark-mode-only)

## Installation

TBD

## Usage

### `changelog init`

Creates an empty `CHANGELOG.md` file in the current directory which looks something like this:

```markdown
# Changelog

## [Unreleased]

### Added
```

### `changelog find`

Finds the nearest `CHANGELOG.md` relative to the current working directory and prints it to the console.
If none is found, keepac will recursively walk upwards the directory tree until it either reaches the root or finds one.

> Note: "The nearest changelog" will be used throughout this readme to refer to this upward search for `CHANGELOG.md` files.

While quite basic, this command can be used in conjunction with other tools to quickly build custom functionality.

### `changelog show`

Renders the nearest changelog right inside your terminal using [charmbracelet/glamour](https://github.com/charmbracelet/glamour).

### `changelog edit`

Opens the nearest changelog inside your `$EDITOR`.

![](https://niclasvaneyk.github.io/keepac/insert.light.gif#gh-light-mode-only)
![](https://niclasvaneyk.github.io/keepac/insert.dark.gif#gh-dark-mode-only)

If you did not set the `$EDITOR` environment variable, commands like `xdg-open` or `open` are used as a fallback.

### `changelog add`

Adds a new entry to one of your sections in the changelog.

Now we arrive at the more useful features of keepac.
Most of the edits to your changelog are additive and will most likely touch the next release.
This is why the keepachangelog guidelines suggest keeping the `[Unreleased]` section right at the top, so you don't have to scroll all the way to the bottom.

By default we assume you want to add to the next release, so running

```shell
changelog add
```

will open up either your `$EDITOR` or an inline one if the `$EDITOR` environment variable is not set.
Describe your changes, close the editor and an entry .

### `changelog release`

Turns the `[Unreleased]` section into a proper versioned release.

You may pass a version adhering to SemVer or use the `--major`, `--minor`, `--patch` flags.
Note that there must be prior releases in order to use these!
By default the current date is used as the realease date, but you may override this using the `--date` option.
