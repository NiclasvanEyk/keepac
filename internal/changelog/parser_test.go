package changelog

import (
	"fmt"
	"testing"

	"gotest.tools/assert"
	is "gotest.tools/assert/cmp"
)

const source = `# Changelog

  ## [1.1.1] - 2023-03-05

  ### Added

  - added something
  - added another

  ### Removed

  - removed something

  ### Changed

  - also changed something`

func TestContentWithinParsedBoundsEqualsSource(t *testing.T) {
	parsed := Parse([]byte(source))
	expected := `## [1.1.1] - 2023-03-05

  ### Added

  - added something
  - added another

  ### Removed

  - removed something

  ### Changed

  - also changed something`

	latestRelease := parsed.Releases.Past[0]
	parsedContent := parsed.ContentWithin(&latestRelease.Bounds)
	fmt.Printf("%v", latestRelease.Bounds)

	assert.Equal(t, expected, parsedContent)
}

func TestParser(t *testing.T) {
	parsed := Parse([]byte(source))

	assert.Equal(t, "Changelog", parsed.Title)
	assert.Assert(t, is.Nil(parsed.Releases.Next))

	assert.Equal(t, 1, len(parsed.Releases.Past))
}
