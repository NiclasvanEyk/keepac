package changelog

import "testing"

func TestItCanSearchForItems(t *testing.T) {
	changelog := Parse([]byte(`
# Changelog

## [Unreleased]

### Added

- Something cool
- Another thing

### Removed

- Support for Go > 1.0
- Support for Windows
	`))

	actual := Search(&changelog, "Windows")
	expected := `
# Changelog

## [Unreleased]

### Removed

- Support for Windows
	`

	if actual != expected {
		t.Errorf("Expected does not match actual:\n\n%s", actual)
	}
}
