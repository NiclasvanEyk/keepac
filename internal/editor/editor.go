package editor

import (
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

func Open(path string) error {
	editor := os.Getenv("EDITOR")
	if editor != "" {
		editorCommand := exec.Command(editor, path)
		editorCommand.Stdout = os.Stdout
		editorCommand.Stdin = os.Stdin
		editorCommand.Stderr = os.Stderr
		err := editorCommand.Run()
		if err != nil {
			return fmt.Errorf("failed to open file with $EDITOR: %s", err)
		}
		return nil
	}

	var openCommand *exec.Cmd
	switch runtime.GOOS {
	case "darwin":
		openCommand = exec.Command("open", path)
	case "linux":
		openCommand = exec.Command("xdg-open", path)
	case "windows":
		openCommand = exec.Command("cmd", "/c", "start", path)
	default:
		return fmt.Errorf("unsupported operating system: %s", runtime.GOOS)
	}

	err := openCommand.Start()
	if err != nil {
		return fmt.Errorf("failed to open file with default command: %s", err)
	}

	return nil
}

func Prompt(initialContents string, description string) (string, error) {
	tempFile, err := os.CreateTemp("", "keepac-input.txt")
	if err != nil {
		return "", fmt.Errorf("failed to create temporary file: %s", err)
	}
	defer os.Remove(tempFile.Name())

	contents := initialContents + "\n\n\n" + description
	_, err = tempFile.WriteString(contents)
	if err != nil {
		return "", fmt.Errorf("failed to write prompt to temporary file: %s", err)
	}

	if err = tempFile.Close(); err != nil {
		return "", fmt.Errorf("failed to close temporary file: %s", err)
	}

	err = Open(tempFile.Name())
	if err != nil {
		return "", err
	}

	data, err := os.ReadFile(tempFile.Name())
	if err != nil {
		return "", fmt.Errorf("failed to read file: %s", err)
	}

	response := string(data)
	response = strings.TrimSpace(response)
	if strings.HasSuffix(response, description) {
		response = strings.TrimSuffix(response, description)
		response = strings.TrimSpace(response)
	}

	return response, nil
}
