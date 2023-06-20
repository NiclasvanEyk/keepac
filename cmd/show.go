package cmd

import (
	"fmt"
	"os"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

var shouldShowPlain bool

// showCmd represents the show command
var showCmd = &cobra.Command{
	Use:   "show [VERSION|latest|next|unreleased]",
	Short: "Displays the contents of the nearest changelog.",
	Long: `Displays the contents of the nearest changelog.

	If a VERSION (e.g. "1.2.3") is specified, only the release notes for that given version will be shown.
	Instead of a specific version you can also use one of the following aliases:
	- "latest" will show the latest release
	- "next" or "unreleased" will show the contents of the [Unreleased]
	`,
	Args: cobra.MaximumNArgs(1),
	ValidArgsFunction: func(
		cmd *cobra.Command,
		args []string,
		toComplete string,
	) ([]string, cobra.ShellCompDirective) {
		matchingVersions, directive := clog.CompleteReleasesAsFirstArgument(cmd, args, toComplete)
		if matchingVersions == nil {
			return matchingVersions, directive
		}

		// We also support 'latest' as an alias
		matchingVersions = append(matchingVersions, "latest")
		matchingVersions = append(matchingVersions, "next")
		matchingVersions = append(matchingVersions, "unreleased")

		return matchingVersions, cobra.ShellCompDirectiveNoFileComp
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		path, err := clog.ResolvePathToChangelog()
		if err != nil {
			return err
		}

		source, err := os.ReadFile(path)
		if err != nil {
			return err
		}

		if len(args) == 0 {
			if shouldShowPlain {
				fmt.Print(string(source))
				return nil
			}

			return clog.Show(string(source))
		}

		changelog := clog.Parse(source)
		bounds, err := findReleaseBounds(args[0], &changelog)
		if err != nil {
			return err
		}

		contents := changelog.ContentWithin(bounds)

		if shouldShowPlain {
			fmt.Print(contents)
			return nil
		}

		return clog.Show(contents)
	},
}

func init() {
	rootCmd.AddCommand(showCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// showCmd.PersistentFlags().String("foo", "", "A help for foo")

	showCmd.Flags().BoolVarP(&shouldShowPlain, "plain", "p", false, "Only print the raw contents, without terminal decorations")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// showCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func isAlias(alias string) bool {
	versionAliases := []string{
		"latest",
		"next",
		"unreleased",
	}
	for _, knownAlias := range versionAliases {
		if alias == knownAlias {
			return true
		}
	}

	return false
}

func findReleaseBounds(versionOrAlias string, changelog *clog.Changelog) (*clog.Bounds, error) {
	if isAlias(versionOrAlias) {
		alias := versionOrAlias

		if alias == "latest" {
			if len(changelog.Releases.Past) < 1 {
				return nil, fmt.Errorf("Cannot show latest release, since there are none")
			}

			return &changelog.Releases.Past[0].Bounds, nil
		}

		if alias == "next" || alias == "unreleased" {
			nextRelease := changelog.Releases.Next
			if nextRelease == nil {
				return nil, fmt.Errorf("Cannot show next release, since there is none")
			}

			return &nextRelease.Bounds, nil
		}

		return nil, fmt.Errorf("Unknown version or alias '%s'")
	}

	version := versionOrAlias
	release := changelog.FindRelease(version)
	if release == nil {
		return nil, fmt.Errorf("Release '%s' not found", version)
	}

	return &release.Bounds, nil
}
