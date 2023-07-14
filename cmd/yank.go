package cmd

import (
	"fmt"
	"os"
	"strings"

	"github.com/spf13/cobra"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
)

var yankCmd = &cobra.Command{
	Use:     "yank",
	Aliases: []string{"yeet"},
	Short:   "Marks the specified release as yanked",
	Long:    `As described by https://keepachangelog.com, yanked releases are versions that had to be pulled because of a serious bug or security issue.`,
	Args:    cobra.ExactArgs(1),
	ValidArgsFunction: func(
		cmd *cobra.Command,
		args []string,
		toComplete string,
	) ([]string, cobra.ShellCompDirective) {
		if len(args) != 0 {
			return nil, cobra.ShellCompDirectiveNoFileComp
		}

		changelog, _, err := clog.ResolveChangelog()
		if err != nil {
			return nil, cobra.ShellCompDirectiveNoFileComp
		}

		matchingVersions := make([]string, 0)
		for _, release := range changelog.Releases.Past {
			if !release.Yanked && strings.HasPrefix(release.Version, toComplete) {
				matchingVersions = append(matchingVersions, release.Version)
			}
		}

		return matchingVersions, cobra.ShellCompDirectiveNoFileComp
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, changelogPath, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		target := args[0]

		newSource, err := changelog.Yank(target)
		if err != nil {
			return err
		}

		err = os.WriteFile(changelogPath, []byte(newSource), 0o774)
		if err != nil {
			return err
		}

		fmt.Printf("Marked '%s' as yanked!\n", target)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(yankCmd)
}
