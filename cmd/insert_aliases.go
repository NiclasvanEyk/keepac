package cmd

import (
	"fmt"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

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

func init() {
	rootCmd.AddCommand(alias("add", nil, clog.Added))
	rootCmd.AddCommand(alias("fix", nil, clog.Fixed))
	rootCmd.AddCommand(alias("change", []string{"cha"}, clog.Changed))
	rootCmd.AddCommand(alias("remove", []string{"rm", "rem"}, clog.Removed))
	rootCmd.AddCommand(alias("deprecate", []string{"dep"}, clog.Deprecated))
	rootCmd.AddCommand(alias("secure", []string{"sec", "security"}, clog.Security))
}
