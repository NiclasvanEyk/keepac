# `keepac`

An opiniated way of managing changelogs adhering to the [keepachangelog](https://keepachangelog.com/) guidelines.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/demo.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/demo.light.gif" loading="lazy">
</picture>

## Installation

### Pre-built Binaries

This section contains a few copy-pastable gists that should download and install the latest version of the `changelog` binary to `/usr/local/bin`. If your arch/OS is not listed here, you may be able to download it from the [GitHub Releases Page](https://github.com/NiclasvanEyk/keepac/releases).

#### Linux

```shell
curl -sL https://github.com/NiclasvanEyk/keepac/releases/latest/download/keepac_Linux_x86_64.tar.gz | tar -xz && mv changelog /usr/local/bin/ && changelog --help
```

### MacOS - Homebrew

```shell
brew install niclasvaneyk/keepac/keepac
```

> Note: You likely need to open a new shell instance after the installation succeeds in order for the completions to work properly

### From Source

If you are not able to download and install a pre-built version, you may build one yourself by cloning this repository and running

```shell
go build -o changelog
```

## Usage

### Reading

#### `changelog find`

Finds the nearest `CHANGELOG.md` relative to the current working directory and prints it to the console.
If none is found, keepac will recursively walk upwards the directory tree until it either reaches the root or finds one.

> Note: "The nearest changelog" will be used throughout this readme to refer to this upward search for `CHANGELOG.md` files.

While quite basic, this command can be used in conjunction with other tools to quickly build custom functionality.

#### `changelog show`

Renders the nearest changelog right inside your terminal using [charmbracelet/glamour](https://github.com/charmbracelet/glamour).

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/show.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/show.light.gif" loading="lazy">
</picture>

> Since keepac uses `glamour` to render markdown, you can theme its output by setting the `GLAMOUR_STYLE` environment variable to one of the [available styles](https://github.com/charmbracelet/glamour/tree/master/styles/gallery) or [create your own](https://github.com/charmbracelet/glamour/tree/master/styles).

#### `changelog search`

Searches for changes matching the given search query.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/search.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/search.light.gif" loading="lazy">
</picture>

Matches are displayed with contextual information, such as the version the change was released in and its type of change.

### Writing

#### `changelog init`

Creates an empty `CHANGELOG.md` file in the current directory which looks something like this:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
```

#### `changelog edit`

Opens the nearest changelog inside your `$EDITOR`.

If you did not set the `$EDITOR` environment variable, commands like `xdg-open` or `open` are used as a fallback.

#### `changelog insert`

Adds a new entry to one of your sections in the changelog.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/insert.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/insert.light.gif" loading="lazy">
</picture>

Now we arrive at the more useful features of keepac.
Most of the edits to your changelog are additive and will most likely touch the next release.
This is why the keepachangelog guidelines suggest keeping the `[Unreleased]` section right at the top, so you don't have to scroll all the way to the bottom.

By default we assume you want to add to the next release, so running

```shell
changelog insert
```

will open up either your `$EDITOR` or an inline one if the `$EDITOR` environment variable is not set.
Describe your changes, close the editor and an entry .

#### `changelog release`

Turns the `[Unreleased]` section into a proper versioned release.

You may pass a version adhering to SemVer or use the `--major`, `--minor`, `--patch` flags.
Note that there must be prior releases in order to use these!
By default the current date is used as the realease date, but you may override this using the `--date` option.

#### `changelog yank <version>`

Marks the specified released as yanked`. To cite [Keep a Changelog](https://keepachangelog.com/en/1.1.0/#yanked):

> Yanked releases are versions that had to be pulled because of a serious bug or security issue. Often these versions don't even appear in change logs. They should. This is how you should display them:
>
> ```markdown
> ## [0.0.5] - 2014-12-13 [YANKED]
> ```
>
> The [YANKED] tag is loud for a reason. It's important for people to notice it. Since it's surrounded by brackets it's also easier to parse programmatically.
