package changelog

import (
	"fmt"

	"github.com/charmbracelet/glamour"
)

func Show(contents string) error {
	renderer, _ := glamour.NewTermRenderer(
		// detect background color and pick either the default dark or light theme
		glamour.WithAutoStyle(),
		glamour.WithEnvironmentConfig(),
	)

	out, err := renderer.Render(contents)
	if err != nil {
		return err
	}

	fmt.Print(out)
	return nil
}
