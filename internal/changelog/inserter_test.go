package changelog

import "testing"

func scenario(
	t *testing.T,
	source string,
	changeType ChangeType,
	addition string,
	expected string,
) {
	changelog := Parse([]byte(source))
	after := changelog.AddItem(changeType, addition)

	if after != expected {
		t.Errorf("Resulting changelog did not match expectations:\n\n'%v'\n\n is not\n\n'%v'", after, expected)
	}
}

func TestAddToCompletelyEmptyChangelog(t *testing.T) {
	source := "# Changelog"
	changeType := Added
	addition := "- New Thing"
	expected := `# Changelog

## [Unreleased]

### Added

- New Thing`

	scenario(t, source, changeType, addition, expected)
}

func TestAddToJustAfterRelease(t *testing.T) {
	source := `# Changelog

## [1.0.0] - 2020-01-01

### Added

- New Thing`

	changeType := Added
	addition := "- Another New Thing"
	expected := `# Changelog

## [Unreleased]

### Added

- Another New Thing

## [1.0.0] - 2020-01-01

### Added

- New Thing`

	scenario(t, source, changeType, addition, expected)
}

func TestAddToExistingSectionInNextRelease(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Added

- Something`
	changeType := Added
	addition := "- Another New Thing"
	expected := `# Changelog

## [Unreleased]

### Added

- Something
- Another New Thing`

	scenario(t, source, changeType, addition, expected)
}

func TestAppendToExistingSectionInNextReleaseWithoutPastReleases(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Added

- First
- Second
- Third`
	changeType := Added
	addition := "- Fourth"
	expected := `# Changelog

## [Unreleased]

### Added

- First
- Second
- Third
- Fourth`

	scenario(t, source, changeType, addition, expected)
}

func TestAddToNewSectionInNextReleaseWithoutPastReleases(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Added

- Something`
	changeType := Changed
	addition := "- Another New Thing"
	expected := `# Changelog

## [Unreleased]

### Added

- Something
- Another New Thing`

	scenario(t, source, changeType, addition, expected)
}

func TestAddNewAddedSectionAboveRemovedOne(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Removed

- Something
- Something else

## [1.1.0] - 2020-01-01

### Added

- Something`
	changeType := Added
	addition := "- Something new"
	expected := `# Changelog

## [Unreleased]

### Added

- Something new

### Removed

- Something
- Something else

## [1.1.0] - 2020-01-01

### Added

- Something`

	scenario(t, source, changeType, addition, expected)
}

func TestAddNewDeprecatedSectionBetweenAddedAndRemovedOnes(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Added

- Something new

### Removed

- Something
- Something else

## [1.1.0] - 2020-01-01

### Added

- Something`
	changeType := Deprecated
	addition := "- Something that will be removed"
	expected := `# Changelog

## [Unreleased]

### Added

- Something new

### Deprecated

- Something that will be removed

### Removed

- Something
- Something else

## [1.1.0] - 2020-01-01

### Added

- Something`

	scenario(t, source, changeType, addition, expected)
}

func TestInsertsAfterEmptyButExistingUnreleasedSection(t *testing.T) {
	source := `# Changelog

## [Unreleased]

## [1.1.0] - 2020-01-01

### Added

- Something`
	changeType := Added
	addition := "- Something"
	expected := `# Changelog

## [Unreleased]

### Added

- Something

## [1.1.0] - 2020-01-01

### Added

- Something`

	scenario(t, source, changeType, addition, expected)
}

func TestInsertsAfterEmptyButExistingUnreleasedSectionWithoutAnyPastReleases(t *testing.T) {
	source := `# Changelog

## [Unreleased]
`
	changeType := Added
	addition := "- Something"
	expected := `# Changelog

## [Unreleased]

### Added

- Something`

	scenario(t, source, changeType, addition, expected)
}

func TestInsertsCorrectlyRegressionTest(t *testing.T) {
	source := `# Changelog

## [Unreleased]

### Added

- The initial version
- new entry
- a really cool new feature
- a really cool new feature
- a really cool new feature
- something
- Something Why does this not get inserted at the end?
- Something This is unexpected
- Something really good
- foo bar foo
- Something`

	addition := "- New item at the end"
	expected := `# Changelog

## [Unreleased]

### Added

- The initial version
- new entry
- a really cool new feature
- a really cool new feature
- a really cool new feature
- something
- Something Why does this not get inserted at the end?
- Something This is unexpected
- Something really good
- foo bar foo
- Something
- New item at the end`

	scenario(t, source, Added, addition, expected)
}
