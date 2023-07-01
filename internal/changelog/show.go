package changelog

import (
	"fmt"
	"os"

	"github.com/charmbracelet/glamour"
	"golang.org/x/crypto/ssh/terminal"
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
	width, _, err := terminal.GetSize(current)
	if err != nil {
		return 80
	}

	if width > 80 {
		return 80
	}

	return width
}
