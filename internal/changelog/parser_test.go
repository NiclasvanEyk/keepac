package changelog

import "testing"

func TestParser(t *testing.T) {
	source := `# Changelog

  ## [1.1.1] - 2023-03-05

  ### Added

  - added something
  - added another

  ### Removed

  - removed something

  ### Changed

  - also changed something`
	Parse([]byte(source))
}
