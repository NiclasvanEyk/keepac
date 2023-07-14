package cmd

import (
	"fmt"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/spf13/cobra"
)

var searchCmd = &cobra.Command{
	Use:     "search [query]",
	Aliases: []string{"grep"},
	Short:   "Searches for strings in the nearest changelog and prints matches within their context",
	Args:    cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, _, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		query := args[0]
		result := clog.Search(changelog, query)

		if result == "" {
			fmt.Println("Nothing matched your query!")
		}

		return clog.Show(result + "\n")
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
}
