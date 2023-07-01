/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
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
	isMajor bool
	isMinor bool
	isPatch bool
)

// releaseCmd represents the release command
var releaseCmd = &cobra.Command{
	Use:   "release",
	Short: "Turns the [Unreleased] section into a proper release",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	RunE: func(cmd *cobra.Command, args []string) error {
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

		var previousVersion semver.Version
		if len(changelog.Releases.Past) > 0 {
			rawVersion := changelog.Releases.Past[0].Version
			previousVersion = semver.MustParse(rawVersion)
		} else {
			previousVersion = semver.Version{Major: 0, Minor: 0, Patch: 0}
		}

		timestamp := time.Now().Format(time.DateOnly)
		version := getNextVersion(&previousVersion)
		newHeadline := fmt.Sprintf("[%s] - %s", version, timestamp)

		bounds := nextRelease.HeadlineBounds
		newSource := changelog.ReplacedWithinBounds(bounds, newHeadline)

		err = os.WriteFile(changelogPath, []byte(newSource), 0o774)
		if err != nil {
			return err
		}

		relevantSection := newSource[nextRelease.Bounds.Start:nextRelease.Bounds.Stop]
		return clog.Show(relevantSection)
	},
}

func init() {
	rootCmd.AddCommand(releaseCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// releaseCmd.PersistentFlags().String("foo", "", "A help for foo")

	releaseCmd.Flags().BoolVarP(&isMajor, "major", "", false, "Release a new major version")
	releaseCmd.Flags().BoolVarP(&isMinor, "minor", "", false, "Release a new minor version")
	releaseCmd.Flags().BoolVarP(&isPatch, "patch", "", false, "Release a new patch version")
	releaseCmd.MarkFlagsMutuallyExclusive("major", "minor", "patch")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// releaseCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func getNextVersion(prev *semver.Version) string {
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
