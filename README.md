# `keepac`

An opiniated way of managing changelogs adhering to the [keepachangelog](https://keepachangelog.com/) guidelines.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/demo.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/demo.light.gif" loading="lazy">
</picture>

## Installation

### Homebrew

The preferred method, since you receive updates via `brew update` and it works on MacOS, Linux and WSL.
Shell completions are installed automatically, so tabbing for command or argument completions just works.

```shell
brew install niclasvaneyk/keepac/keepac
```

> Note: You likely need to open a new shell instance after the installation succeeds in order for the completions to work properly

### Other

Pre-built binaries and bundles can be downloaded from the [GitHub Releases Page](https://github.com/NiclasvanEyk/keepac/releases).
It contains `.deb`, `.rpm`, `.exe` and other formats for a variety of CPU architectures.
These do not get automatic updates and depending on the format you may need to install the shell completions yourself.

If you are not able to download and install a pre-built version, you may build one yourself by cloning this repository and running `go build -o changelog`.

## Usage

Writing changelogs is not rocket science, but the process definitely feels more manual than other workflows.

1. You need to find the `CHANGELOG.md` in your project and open it in your editor:

   ```markdown
   # Changelog

   ## [2.3.4] - 2023-06-06

   ...
   ```

2. You then create a new section for the next release

   ```diff
   # Changelog

   + ## [Unreleased]

   ## [2.3.4] - 2023-06-06

   ...
   ```

3. another one for bug fixes

   ```diff
   # Changelog

   ## [Unreleased]

   + ### Fixed

   ## [2.3.4] - 2023-06-06

   ...
   ```

4. and finally document your fix:

   ```diff
   # Changelog

   ## [Unreleased]

   ### Fixed

   + - A bug that lead to X when doing Y

   ## [2.3.4] - 2023-06-06

   ...
   ```

5. then you save the document, close your editor and you are done.

Presenting this as a list makes it look like more effort than it actually is, but each step requires tiny ammounts of mental effort and could prevent someone from keeping a changelog.

With `keepac` you could have done all of this by running

```shell
changelog fixed "A bug that lead to X when doing Y"
```

inside your terminal, similar to how you would do it with `git`.

Missing sections are added on-demand and sections are always added in the same order.
It can be run from anywhere inside your project, since it recursively searches upward the directory tree for a `CHANGELOG.md` file.
Less thinking _how_ to do something and more focus on the _what_.

## Reference

### Reading

#### `changelog find`

Finds the nearest `CHANGELOG.md` relative to the current working directory and prints it to the console.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://niclasvaneyk.github.io/keepac/find.dark.gif">
  <img src="https://niclasvaneyk.github.io/keepac/find.light.gif" loading="lazy">
</picture>

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

Marks the specified released as yanked. To cite [Keep a Changelog](https://keepachangelog.com/en/1.1.0/#yanked):

> Yanked releases are versions that had to be pulled because of a serious bug or security issue. Often these versions don't even appear in change logs. They should. This is how you should display them:
>
> ```markdown
> ## [0.0.5] - 2014-12-13 [YANKED]
> ```
>
> The [YANKED] tag is loud for a reason. It's important for people to notice it. Since it's surrounded by brackets it's also easier to parse programmatically.
