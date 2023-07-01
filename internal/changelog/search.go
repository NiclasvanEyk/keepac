package changelog

import (
	"strings"
)

func Search(changelog *Changelog, query string) string {
	output := make([]string, 0)

	nextRelease := changelog.Releases.Next
	if nextRelease != nil {
		includedRelease := false
		for _, section := range nextRelease.Sections {
			includedSection := false
			for _, item := range section.Items {
				text := changelog.ContentWithin(&item.Bounds)
				if strings.Contains(text, query) {
					if !includedRelease {
						includedRelease = true
						output = append(output, "## "+changelog.ContentWithin(&nextRelease.HeadlineBounds))
					}

					if !includedSection {
						includedSection = true
						output = append(output, "### "+ChangeTypeLabel(section.Type))
					}

					output = append(output, "- "+text)
				}
			}
		}
	}

	for _, release := range changelog.Releases.Past {
		includedRelease := false
		for _, section := range release.Sections {
			includedSection := false
			for _, item := range section.Items {
				text := changelog.ContentWithin(&item.Bounds)
				if strings.Contains(text, query) {
					if !includedRelease {
						includedRelease = true
						output = append(output, "## "+changelog.ContentWithin(&release.HeadlineBounds))
						output = append(output, "")
					}

					if !includedSection {
						includedSection = true
						output = append(output, "### "+ChangeTypeLabel(section.Type))
						output = append(output, "")
					}
					output = append(output, "- "+text)
				}
			}
		}
	}

	return strings.Join(output, "\n")
}
