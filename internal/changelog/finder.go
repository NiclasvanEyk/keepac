package changelog

import (
	"fmt"
	"io/fs"
	"io/ioutil"
	"os"
	"path/filepath"
)

func FindChangelogIn(directory string) (string, bool) {
	changelogPath := filepath.Join(directory, "CHANGELOG.md")
	_, err := os.Stat(changelogPath)
	if err != nil {
		return "", false
	}

	return changelogPath, true
}

func hikeDir(start string, callback func(string) error) {
	directory := start
	for {
		result := callback(directory)
		if result == fs.SkipAll {
			break
		}

		parent := filepath.Dir(directory)
		rootDirectoryReached := parent == directory
		if rootDirectoryReached {
			break
		}

		directory = parent
	}
}

func ResolvePathToChangelog() (string, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return "", err
	}

	changelogPath, wasFound := "", false
	hikeDir(cwd, func(directory string) error {
		changelogPath, wasFound = FindChangelogIn(directory)
		if wasFound {
			return fs.SkipAll
		}
		return nil
	})

	if wasFound {
		return changelogPath, nil
	}

	return "", fmt.Errorf("CHANGELOG.md not found")
}

func ResolveChangelog() (*Changelog, string, error) {
	filename, err := ResolvePathToChangelog()
	if err != nil {
		// TODO: if none exists, maybe ask to create one? Might not make sense though
		return nil, "", err
	}

	contents, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, "", err
	}

	changelog := Parse(contents)

	return &changelog, filename, nil
}
