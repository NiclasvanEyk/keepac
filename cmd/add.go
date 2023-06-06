package cmd

import (
	// "fmt"

	"os"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/editor"

	"github.com/spf13/cobra"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Adds a new entry to a specified section of the next release",
	Long: `Adds a new entry to a specified section of the next release using your preferred editor.

  Honors the $EDITOR environment variable and falls back to xdg-open (linux), open (mac) or cmd /c start (windows).`,
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, filename, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		response, err := editor.Prompt("- ", "<!-- Add your changes above. Don't worry, this line will be excluded from the final output. -->")
		if err != nil {
			return err
		}

		// TODO: Support adding a section via a flag (--added, --changed, deleted)
		// TODO: Prompt for the section if none was specified
		// TODO: Create the section, if it was not present previously
		//
		// TODO: Add the response to the specified section

		newSource := changelog.AddItem(clog.Added, response)

		return os.WriteFile(filename, []byte(newSource), 0774)
	},
}

func init() {
	rootCmd.AddCommand(addCmd)

	addCmd.Flags().BoolP("added", "a", false, "Adds the change to the 'Added' section.")
	addCmd.Flags().BoolP("changed", "c", false, "Adds the change to the 'Changed' section.")
	addCmd.Flags().BoolP("deprecated", "d", false, "Adds the change to the 'Deprecated' section.")
	addCmd.Flags().BoolP("removed", "r", false, "Adds the change to the 'Removed' section.")
	addCmd.Flags().BoolP("fixed", "f", false, "Adds the change to the 'Fixed' section.")
	addCmd.Flags().BoolP("security", "s", false, "Adds the change to the 'Security' section.")
	addCmd.MarkFlagsMutuallyExclusive("added", "changed", "deprecated", "removed", "fixed", "security")
}
