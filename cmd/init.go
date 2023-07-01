/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"os"
	"path"

	clog "github.com/niclasvaneyk/keepac/internal/changelog"

	"github.com/spf13/cobra"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Creates a CHANGELOG.md with an empty [Unreleased] section",
	RunE: func(cmd *cobra.Command, args []string) error {
		cwd, err := os.Getwd()
		if err != nil {
			return err
		}

		changelogPath, wasFound := clog.FindChangelogIn(cwd)
		if wasFound {
			return fmt.Errorf("changelog already exists at %s", changelogPath)
		}

		changelogPath = path.Join(cwd, "CHANGELOG.md")
		changelogContents := `# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
`

		err = os.WriteFile(changelogPath, []byte(changelogContents), 0o774)
		if err != nil {
			return err
		}

		fmt.Printf("Initialized empty changelog at %s:\n", changelogPath)
		return clog.Show(changelogContents)
	},
}

func init() {
	rootCmd.AddCommand(initCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// initCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// initCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
