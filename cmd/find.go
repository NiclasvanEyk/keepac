package cmd

import (
	"fmt"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

var findCmd = &cobra.Command{
	Use:     "find",
	Aliases: []string{"path"},
	Short:   "Attempts to find the nearest CHANGELOG.md file relative to the current working directory",
	RunE: func(cmd *cobra.Command, args []string) error {
		path, err := clog.ResolvePathToChangelog()
		if err != nil {
			return err
		}

		if err != nil {
			return err
		}

		fmt.Println(path)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(findCmd)
}
