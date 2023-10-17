package changelog

import (
	"fmt"
	"os"
	"strconv"

	"github.com/charmbracelet/glamour"
	"golang.org/x/term"
)

func (changelog *Changelog) ContentWithin(bounds *Bounds) string {
	return changelog.source[bounds.Start:bounds.Stop]
}

func Show(contents string) error {
	renderer, err := glamour.NewTermRenderer(
		glamour.WithAutoStyle(),
		glamour.WithEnvironmentConfig(),
		glamour.WithWordWrap(getWordWrapLimit()),
	)
	if err != nil {
		return err
	}

	out, err := renderer.Render(contents)
	if err != nil {
		return err
	}

	fmt.Print(out)
	return nil
}

func getWordWrapLimit() int {
	current := int(os.Stdin.Fd())
	// When wrapping at exatcly 85, the default contents generated
	// `changelog init` wrap nicely:
	//
	//    Changelog
	//
	//    All notable changes to this project will be documented in this file.
	//
	//    The format is based on Keep a Changelog https://keepachangelog.com/en/1.0.0/, and
	//    this project adheres to Semantic Versioning https://semver.org/spec/v2.0.0.html.
	//
	// Anything less than 85 leads to the links being broken up, which does not
	// look very good.
	fallback := 85

	preferredRaw := os.Getenv("KEEPAC_WRAP_AT")
	preferred, err := strconv.Atoi(preferredRaw)
	if err != nil {
		preferred = fallback
	}

	width, _, err := term.GetSize(current)
	if err != nil {
		return fallback
	}

	if width > preferred {
		return preferred
	}

	return width
}
