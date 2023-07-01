package cmd

import (
	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/spf13/cobra"
)

// searchCmd represents the grep command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Searches for strings in the nearest changelog and prints matches within their context",
	Long:  ``,
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, _, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
}
