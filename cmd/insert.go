package cmd

import (
	// "fmt"

	"os"
	"strings"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/editor"
	"github.com/niclasvaneyk/keepac/internal/tui"

	"github.com/spf13/cobra"
)

var changeTypeAdded bool
var changeTypeChanged bool
var changeTypeDeprecated bool
var changeTypeRemoved bool
var changeTypeFixed bool
var changeTypeSecurity bool

// insertCmd represents the insert command
var insertCmd = &cobra.Command{
	Use:     "insert",
	Aliases: []string{"i"},
	Short:   "Inserts a new entry to a specified section of the next release",
	Long: `Inserts a new entry to a specified section of the next release using your preferred editor.

  Honors the $EDITOR environment variable and falls back to xdg-open (linux), open (mac) or cmd /c start (windows).`,
	RunE: func(cmd *cobra.Command, args []string) error {
		changelog, filename, err := clog.ResolveChangelog()
		if err != nil {
			return err
		}

		var changeType clog.ChangeType
		if changeTypeAdded {
			changeType = clog.Added
		} else if changeTypeChanged {
			changeType = clog.Changed
		} else if changeTypeDeprecated {
			changeType = clog.Deprecated
		} else if changeTypeRemoved {
			changeType = clog.Removed
		} else if changeTypeFixed {
			changeType = clog.Fixed
		} else if changeTypeSecurity {
			changeType = clog.Security
		} else {
			changeType = chooseChangeType()
			if changeType == clog.Unknown {
				return nil
			}
		}

		var response string
		if len(args) > 0 {
			response = strings.Join(args, " ")
		} else {
			response, err = editor.Prompt("- ", "<!-- Add your changes above. Don't worry, this line will be excluded from the final output. -->")
			if err != nil {
				return err
			}
		}

		response = normalized(response)

		newSource := changelog.AddItem(changeType, response)
		return os.WriteFile(filename, []byte(newSource), 0774)
	},
}

func init() {
	rootCmd.AddCommand(insertCmd)

	insertCmd.Flags().BoolVarP(&changeTypeAdded, "added", "a", false, "Adds the change to the 'Added' section.")
	insertCmd.Flags().BoolVarP(&changeTypeChanged, "changed", "c", false, "Adds the change to the 'Changed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeDeprecated, "deprecated", "d", false, "Adds the change to the 'Deprecated' section.")
	insertCmd.Flags().BoolVarP(&changeTypeRemoved, "removed", "r", false, "Adds the change to the 'Removed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeFixed, "fixed", "f", false, "Adds the change to the 'Fixed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeSecurity, "security", "s", false, "Adds the change to the 'Security' section.")
	insertCmd.MarkFlagsMutuallyExclusive("added", "changed", "deprecated", "removed", "fixed", "security")
}

func chooseChangeType() clog.ChangeType {
	choice := tui.Choice("What type of change do you want to document?", []string{
		"Added",
		"Changed",
		"Deprecated",
		"Removed",
		"Fixed",
		"Security",
	})

	return clog.ParseChangeType(choice)
}

func normalized(response string) string {
	normalized := strings.TrimSpace(response)

	if strings.HasPrefix(normalized, "- ") {
		return normalized
	}

	return "- " + normalized
}
