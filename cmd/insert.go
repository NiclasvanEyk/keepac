package cmd

import (
	"fmt"
	"os"
	"strings"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"
	"github.com/niclasvaneyk/keepac/internal/editor"
	"github.com/niclasvaneyk/keepac/internal/tui"

	"github.com/spf13/cobra"
)

var (
	changeTypeAdded      bool
	changeTypeChanged    bool
	changeTypeDeprecated bool
	changeTypeRemoved    bool
	changeTypeFixed      bool
	changeTypeSecurity   bool
)

func runInsertCmd(changelog *clog.Changelog, args []string, filename string, changeType clog.ChangeType) error {
	response, err := promptForDescription(args)
	if err != nil {
		return err
	}

	newSource := changelog.AddItem(changeType, response)
	err = os.WriteFile(filename, []byte(newSource), 0o774)
	if err != nil {
		return err
	}

	return clog.Show(viewAfterInsertion(newSource, changeType))
}

func promptForDescription(args []string) (string, error) {
	var response string
	var err error

	if len(args) > 0 {
		response = strings.Join(args, " ")
	} else {
		response, err = editor.Prompt("- ", "<!-- Add your changes above. Don't worry, this line will be excluded from the final output. -->")
		if err != nil {
			return "", err
		}

		if response == "-" {
			return "", fmt.Errorf("no description provided")
		}
	}

	return normalized(response), err
}

func viewAfterInsertion(newSource string, changeType clog.ChangeType) string {
	newChangelog := clog.Parse([]byte(newSource))
	editedSection := newChangelog.Releases.Next.FindSection(changeType)
	if editedSection == nil {
		return ""
	}

	items := make([]string, 0)
	const MAX_ITEMS_SHOWN = 4

	offset := 0
	if len(editedSection.Items) > MAX_ITEMS_SHOWN {
		offset = 1
		items = append(items, "- ...")
	}

	index := (len(editedSection.Items) - MAX_ITEMS_SHOWN) + offset
	if index < 0 {
		index = 0
	}
	for ; len(items) < MAX_ITEMS_SHOWN && index < len(editedSection.Items); index++ {
		bounds := editedSection.Items[index].Bounds
		items = append(items, "- "+newChangelog.ContentWithin(&bounds))
	}

	headline := "### " + clog.ChangeTypeLabel(changeType)

	return headline + "\n" + strings.Join(items, "\n")
}

var insertCmd = &cobra.Command{
	Use:     "insert",
	Aliases: []string{"i"},
	Short:   "Inserts a new entry to a specified section of the next release",
	Long: `Inserts a new entry to a specified section of the next release using your preferred editor.

Honors the $EDITOR environment variable and falls back to xdg-open (linux), open (mac) or cmd /c start (windows).

If you prefer using a shorter approach to using flags, there are a few shortcut commands available that
insert a new entry into a specific section (values in [] can be ommitted):
- add
- fix
- cha[nge]
- rem[ove] (rm is also available)
- dep[recate]
- sec[ure]`,
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

		return runInsertCmd(changelog, args, filename, changeType)
	},
}

func init() {
	rootCmd.AddCommand(insertCmd)
	rootCmd.AddCommand(alias("add", nil, clog.Added))
	rootCmd.AddCommand(alias("fix", nil, clog.Fixed))
	rootCmd.AddCommand(alias("change", []string{"cha"}, clog.Changed))
	rootCmd.AddCommand(alias("remove", []string{"rm", "rem"}, clog.Removed))
	rootCmd.AddCommand(alias("deprecate", []string{"dep"}, clog.Deprecated))
	rootCmd.AddCommand(alias("secure", []string{"sec", "security"}, clog.Security))

	insertCmd.Flags().BoolVarP(&changeTypeAdded, "added", "a", false, "Adds the change to the 'Added' section.")
	insertCmd.Flags().BoolVarP(&changeTypeChanged, "changed", "c", false, "Adds the change to the 'Changed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeDeprecated, "deprecated", "d", false, "Adds the change to the 'Deprecated' section.")
	insertCmd.Flags().BoolVarP(&changeTypeRemoved, "removed", "r", false, "Adds the change to the 'Removed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeFixed, "fixed", "f", false, "Adds the change to the 'Fixed' section.")
	insertCmd.Flags().BoolVarP(&changeTypeSecurity, "security", "s", false, "Adds the change to the 'Security' section.")
	insertCmd.MarkFlagsMutuallyExclusive("added", "changed", "deprecated", "removed", "fixed", "security")
}

func chooseChangeType() clog.ChangeType {
	choice, _ := tui.Choice("What type of change do you want to document?", []string{
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

func alias(command string, aliases []string, changeType clog.ChangeType) *cobra.Command {
	section := clog.ChangeTypeLabel(changeType)

	return &cobra.Command{
		Use:     command,
		Aliases: aliases,
		Short:   fmt.Sprintf(`Inserts a new entry into the "%s" section of the next release`, section),
		Long: fmt.Sprintf(`Inserts a new entry into the "%s" section of the next release using your preferred editor.

  Honors the $EDITOR environment variable and falls back to xdg-open (linux), open (mac) or cmd /c start (windows).`, section),
		RunE: func(cmd *cobra.Command, args []string) error {
			changelog, filename, err := clog.ResolveChangelog()
			if err != nil {
				return err
			}

			return runInsertCmd(changelog, args, filename, changeType)
		},
	}
}
