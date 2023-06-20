package changelog

import (
	"strings"

	"github.com/spf13/cobra"
)

// Can be passed as ValidArgsFunction to support custom completions if the
// first argument is a released version.
func CompleteReleasesAsFirstArgument(
	cmd *cobra.Command,
	args []string,
	toComplete string,
) ([]string, cobra.ShellCompDirective) {
	if len(args) != 0 {
		return nil, cobra.ShellCompDirectiveNoFileComp
	}

	changelog, _, err := ResolveChangelog()
	if err != nil {
		return nil, cobra.ShellCompDirectiveNoFileComp
	}

	matchingVersions := make([]string, 0)
	for _, release := range changelog.Releases.Past {
		if strings.HasPrefix(release.Version, toComplete) {
			matchingVersions = append(matchingVersions, release.Version)
		}
	}

	return matchingVersions, cobra.ShellCompDirectiveNoFileComp
}
