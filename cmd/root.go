package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:          "changelog",
	SilenceUsage: true,
	Version:      "0.0.6",
	Long: `keepac provides useful tools for working with changelogs.

You can show, search, compare or add new changes from anywhere within your project,
without the needing to open up any editors and manually inserting new markdown 
sections.`,
	RunE: showCmd.RunE,
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.SetHelpCommand(&cobra.Command{Hidden: true})
	rootCmd.CompletionOptions.HiddenDefaultCmd = true
}
