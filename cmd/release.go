package cmd

import (
	"fmt"
	"os"
	"time"

	"github.com/blang/semver/v4"
	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/tui"

	"github.com/spf13/cobra"
)

var (
	isMajor                  bool
	isMinor                  bool
	isPatch                  bool
	manuallySpecifiedVersion string
)

var releaseCmd = &cobra.Command{
	Use:   "release",
	Short: "Turns the [Unreleased] section into a proper release",
	RunE: func(_ *cobra.Command, _ []string) error {
		changelog, changelogPath, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		nextRelease := changelog.Releases.Next
		if nextRelease == nil {
			return fmt.Errorf("%s does not contain an [Unreleased] section", changelogPath)
		}

		if !clog.HasChanges(&nextRelease.Sections) {
			return fmt.Errorf("the [Unreleased] section of %s does not contain any changes", changelogPath)
		}

		version := manuallySpecifiedVersion
		if manuallySpecifiedVersion == "" {
			version = getVersion(changelog)
		}
		timestamp := time.Now().Format(time.DateOnly)
		newHeadline := fmt.Sprintf("[%s] - %s", version, timestamp)

		bounds := nextRelease.HeadlineBounds
		newSource := changelog.ReplacedWithinBounds(bounds, newHeadline)

		err = os.WriteFile(changelogPath, []byte(newSource), 0o774)
		if err != nil {
			return err
		}

		// Since we replaced the headline, simply using the old bounds would cut
		// of some letters at the end
		stop := nextRelease.Bounds.Stop + changelog.DiffLen(newSource)
		relevantSection := newSource[nextRelease.Bounds.Start:stop]

		return clog.Show(relevantSection)
	},
}

func init() {
	rootCmd.AddCommand(releaseCmd)

	releaseCmd.Flags().BoolVar(&isMajor, "major", false, "Release a new major version")
	releaseCmd.Flags().BoolVar(&isMinor, "minor", false, "Release a new minor version")
	releaseCmd.Flags().BoolVar(&isPatch, "patch", false, "Release a new patch version")
	releaseCmd.Flags().StringVarP(&manuallySpecifiedVersion, "version", "v", "", "Manually specify the a version")
	releaseCmd.MarkFlagsMutuallyExclusive("major", "minor", "patch", "version")
}

func getVersion(changelog *clog.Changelog) string {
	var prev semver.Version
	if len(changelog.Releases.Past) > 0 {
		rawVersion := changelog.Releases.Past[0].Version
		prev = semver.MustParse(rawVersion)
	} else {
		prev = semver.Version{Major: 0, Minor: 0, Patch: 0}
	}

	nextMajor := semver.Version{Major: prev.Major + 1, Minor: 0, Patch: 0}.String()
	nextMinor := semver.Version{Major: prev.Major, Minor: prev.Minor + 1, Patch: 0}.String()
	nextPatch := semver.Version{Major: prev.Major, Minor: prev.Minor, Patch: prev.Patch + 1}.String()

	var index int
	if !(isMajor || isMinor || isPatch) {
		_, index = tui.Choice("What type of release do you want to create?", []string{
			fmt.Sprintf("Major (%s)", nextMajor),
			fmt.Sprintf("Minor (%s)", nextMinor),
			fmt.Sprintf("Patch (%s)", nextPatch),
		})
	} else {
		if isMajor {
			index = 0
		} else if isMinor {
			index = 1
		} else {
			index = 2
		}
	}

	if index == 0 {
		return nextMajor
	}

	if index == 1 {
		return nextMinor
	}

	return nextPatch
}
