package changelog

import "strings"

func Search(changelog *Changelog, query string) string {
	matchingBounds := make([]string, 0)

	nextRelease := changelog.Releases.Next
	if nextRelease != nil {
		for _, section := range nextRelease.Sections {
			for _, item := range section.Items {
				if strings.Contains(item, query) {
					matchingBounds = append(matchingBounds, "- "+item)
				}
			}
		}
	}

	for _, release := range changelog.Releases.Past {
		for _, section := range release.Sections {
			for _, item := range section.Items {
				if strings.Contains(item, query) {
					matchingBounds = append(matchingBounds, item)
				}
			}
		}
	}

	return strings.Join(matchingBounds, "\n")
}
