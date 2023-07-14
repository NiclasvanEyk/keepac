package changelog

import (
	"testing"

	"gotest.tools/assert"
	is "gotest.tools/assert/cmp"
)

func TestDiff(t *testing.T) {
	source := `# Changelog

## [0.0.3] - 2020-01-01

### Added

- The third version

## [0.0.2] - 2020-01-01

### Added

- The second version

## [0.0.1] - 2020-01-01

### Added

- The first version`

	changelog := Parse([]byte(source))

	diff, err := changelog.Diff("0.0.2", "0.0.3")
	assert.Assert(t, is.Nil(err))

	expected := `## [0.0.3] - 2020-01-01

### Added

- The third version

## [0.0.2] - 2020-01-01

### Added

- The second version

`
	assert.Equal(t, expected, diff)
}

func TestMerge(t *testing.T) {
	source := `# Changelog

## [0.0.3] - 2020-01-01

### Added

- The third version

## [0.0.2] - 2020-01-01

### Added

- The second version

## [0.0.1] - 2020-01-01

### Added

- The first version`

	changelog := Parse([]byte(source))

	diff, err := changelog.Merge("0.0.2", "0.0.3", false)
	assert.Assert(t, is.Nil(err))

	expected := `# 0.0.2 -> 0.0.3

## Added
- The third version

- The second version`
	assert.Equal(t, expected, diff)
}
