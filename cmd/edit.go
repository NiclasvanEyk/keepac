/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/editor"

	"github.com/spf13/cobra"
)

// editCmd represents the edit command
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

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// editCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// editCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
