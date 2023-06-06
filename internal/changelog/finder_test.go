package changelog

import "testing"

func TestFinder(t *testing.T) {
	path, err := ResolvePathToChangelog()
	if err != nil {
		t.Errorf(err.Error())
	}

	println(path)
}
