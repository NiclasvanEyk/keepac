package cmd

import (
	"strings"
	"testing"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"gotest.tools/assert"
)

func Test_viewAfterInsertion(t *testing.T) {
	tests := []struct {
		name       string
		changeType clog.ChangeType
		newSource  string
		want       string
	}{
		{
			name:       "Truncated",
			changeType: clog.Added,
			newSource: `
# Changelog

## [Unreleased]

### Added

- First Entry
- Second Entry
- Third Entry
- Fourth Entry
- Fifth Entry
- Sixth Entry
- Seventh Entry
- New Entry
`,
			want: `
### Added
- ...
- Sixth Entry
- Seventh Entry
- New Entry
`,
		},
		{
			name:       "First Entry",
			changeType: clog.Added,
			newSource: `
# Changelog

## [Unreleased]

### Added

- New Entry
`,
			want: `
### Added
- New Entry
`,
		},
		{
			name:       "Max Items",
			changeType: clog.Added,
			newSource: `
# Changelog

## [Unreleased]

### Added

- First Entry
- Second Entry
- Third Entry
- New Entry
`,
			want: `
### Added
- First Entry
- Second Entry
- Third Entry
- New Entry
`,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := viewAfterInsertion(tt.newSource, tt.changeType); got != tt.want {
				expected := strings.TrimSpace(tt.want)
				actual := strings.TrimSpace(got)
				assert.Equal(t, expected, actual)
			}
		})
	}
}
