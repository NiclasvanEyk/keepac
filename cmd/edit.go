package cmd

import (
	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/editor"

	"github.com/spf13/cobra"
)

var editCmd = &cobra.Command{
	Use:   "edit",
	Short: "Opens the nearest changelog in a text editor.",
	Long: `Opens the nearest changelog in a text editor.

  Honors the $EDITOR environment variable and falls back to xdg-open (linux), open (mac) or cmd /c start (windows).`,
	RunE: func(cmd *cobra.Command, args []string) error {
		filename, err := clog.ResolvePathToChangelog()
		if err != nil {
			return err
		}

		return editor.Open(filename)
	},
}

func init() {
	rootCmd.AddCommand(editCmd)
}
