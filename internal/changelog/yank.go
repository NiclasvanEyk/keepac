package changelog

import "fmt"

func (changelog *Changelog) Yank(version string) (string, error) {
	release := changelog.FindRelease(version)
	if release == nil {
		return "", fmt.Errorf("Release '%s' not found", version)
	}

	if release.Yanked {
		return "", fmt.Errorf("Release '%s' was already yanked", version)
	}

	return changelog.WithAddition(release.HeadlineBounds.Stop, " [YANKED]"), nil
}
