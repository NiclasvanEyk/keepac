package changelog

import (
	"strings"
	"testing"
)

func TestItCanSearchForItems(t *testing.T) {
	changelog := Parse([]byte(`# Changelog

## [Unreleased]

### Added

- Something cool
- Another thing

### Removed

- Support for Go > 1.0
- Support for Windows
	`))

	actual := Search(&changelog, "Windows")
	expected := `## [Unreleased]
### Removed
- Support for Windows
	`

	if strings.TrimSpace(expected) != strings.TrimSpace(actual) {
		t.Errorf("Expected does not match actual:\n\n%s", actual)
	}
}
