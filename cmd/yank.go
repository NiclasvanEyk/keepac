package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
)

// yankCmd represents the yank command
var yankCmd = &cobra.Command{
	Use:               "yank",
	Short:             "Marks the specified release as yanked",
	Long:              `As described by https://keepachangelog.com, yanked releases are versions that had to be pulled because of a serious bug or security issue.`,
	Args:              cobra.ExactArgs(1),
	ValidArgsFunction: clog.CompleteReleasesAsFirstArgument,
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

		err = os.WriteFile(changelogPath, []byte(newSource), 0774)
		if err != nil {
			return err
		}

		fmt.Printf("Marked '%s' as yanked!\n", target)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(yankCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// yankCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// yankCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
