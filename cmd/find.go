package cmd

import (
	"fmt"
	"os"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

// findCmd represents the find command
var findCmd = &cobra.Command{
	Use:       "find",
	Short:     "Attempts to find the nearest CHANGELOG.md file relative to the current working directory",
	ValidArgs: []string{"latest", "next"},
	RunE: func(cmd *cobra.Command, args []string) error {
		path, err := clog.ResolvePathToChangelog()
		if err != nil {
			return err
		}

		if len(args) == 0 {
			fmt.Printf("%s\n", path)
			os.Exit(0)
		}

		excerpt := args[0]

		if excerpt == "latest" {
			println("Only showing latest release...")
		}

		if excerpt == "next" {
			println("Only next latest release...")
		}

		return fmt.Errorf("unknown display mode: %v", excerpt)
	},
}

func init() {
	rootCmd.AddCommand(findCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// findCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// findCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
