package cmd

import (
	"fmt"
	"os"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

var shouldShowPlain bool

// showCmd represents the show command
var showCmd = &cobra.Command{
	Use:   "show",
	Short: "Displays the contents of the nearest changelog.",
	Long:  `Displays the contents of the nearest changelog.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		path, err := clog.ResolvePathToChangelog()
		if err != nil {
			return err
		}

		source, err := os.ReadFile(path)
		if err != nil {
			return err
		}

		if shouldShowPlain {
			fmt.Print(string(source))
			return nil
		}

		return clog.Show(string(source))
	},
}

func init() {
	rootCmd.AddCommand(showCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// showCmd.PersistentFlags().String("foo", "", "A help for foo")

	showCmd.Flags().BoolVarP(&shouldShowPlain, "plain", "p", false, "Only print the raw contents, without terminal decorations")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// showCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
