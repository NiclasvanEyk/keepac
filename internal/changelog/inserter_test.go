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

// Just after release
//===============
// # Changelog
//
// ## [1.1.0] - 2020-01-01        <-- Add before this line
//
// ### Added

// Section exists
//===============
// # Changelog
//
// ## [Unreleased]
//
// ### Added
//
// - Something
// - Something else              <-- Add after this line
//
// ## [1.1.0] - 2020-01-01

// Create Section - Add 'Added' section
//===============
// # Changelog
//
// ## [Unreleased]
//
// ### Removed                   <-- Add before this line
//
// - Something
// - Something else
//
// ## [1.1.0] - 2020-01-01

// Create Section - B
//===============
// # Changelog
//
// ## [Unreleased]
//
// ### Removed                    <-- Add before this line
//
// - Something
// - Something else
//
// ## [1.1.0] - 2020-01-01

// Create Section - C
//===============
// # Changelog
//
// ## [Unreleased]
//
// ### Added
//
// - Something
// - Something else
//
// ### Removed                    <-- Add before this line
//
// - Something
// - Something else
//
// ## [1.1.0] - 2020-01-01

// Create Section - D
//===============
// # Changelog
//
// ## [Unreleased]               <-- Add after this line
//
// ## [1.1.0] - 2020-01-01

// Create Section - D
//===============
// # Changelog
//
// ## [Unreleased]               <-- Add after this line
