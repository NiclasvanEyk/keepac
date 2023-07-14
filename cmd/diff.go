package cmd

import (
	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/spf13/cobra"
)

var merge bool
var prefix bool

// diffCmd represents the diff command
var diffCmd = &cobra.Command{
	Use:     "diff [flags] from to",
	Aliases: []string{"compare"},
	Short:   "View the logs between two versions",
	Args:    cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, _, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		from, to := args[0], args[1]

		var contents string
		if merge {
			contents, err = changelog.Merge(from, to, prefix)
			if err != nil {
				return err
			}
		} else {
			contents, err = changelog.Diff(from, to)
			if err != nil {
				return err
			}
		}

		return clog.Show(contents)
	},
}

func init() {
	rootCmd.AddCommand(diffCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// diffCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// diffCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	diffCmd.Flags().BoolVarP(&merge, "merged", "", false, "Merge sections into a single continuous")
	diffCmd.Flags().BoolVarP(&prefix, "prefixed", "", false, "When --merged is passed, prefix each change with its version number")
}
