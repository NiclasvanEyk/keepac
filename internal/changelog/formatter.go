package changelog

import (
	"fmt"
	"strings"
)

// Shows everything between the two given versions.
func (changelog *Changelog) Diff(from string, to string) (string, error) {
	releases, err := releasesFromTo(changelog, from, to)
	if err != nil {
		return "", err
	}

	start := releases[0].Bounds.Start
	stop := releases[len(releases)-1].Bounds.Stop

	return changelog.ContentWithin(&Bounds{Start: start, Stop: stop}), nil
}

// Merges
func (changelog *Changelog) Merge(from string, to string, prefixItemsWithVersion bool) (string, error) {
	releases, err := releasesFromTo(changelog, from, to)
	if err != nil {
		return "", err
	}

	merged := EmptyMergedSections()
	for _, release := range releases {
		merged.Merge(release)
	}

	rendered := merged.Render(changelog, prefixItemsWithVersion)

	return fmt.Sprintf("# %s -> %s\n\n%s", from, to, rendered), nil
}

// Returns the releases between from and to (inclusively)
func releasesFromTo(changelog *Changelog, from string, to string) ([]Release, error) {
	between := make([]Release, 0)
	beginTracking := false

	for _, release := range changelog.Releases.Past {
		if !beginTracking && release.Version == to {
			beginTracking = true
		}

		if beginTracking {
			between = append(between, release)
		}

		if release.Version == from {
			return between, nil
		}
	}

	if !beginTracking {
		return between, fmt.Errorf("version '%s' does not exist", to)
	}

	return between, fmt.Errorf("version '%s' does not exist", from)
}

type MergedItem struct {
	Release Release
	Item    *Item
}

type MergedSections struct {
	sections [][]MergedItem
}

func EmptyMergedSections() MergedSections {
	return MergedSections{
		sections: make([][]MergedItem, int(LastChangeType())+1),
	}
}

func (merged *MergedSections) Merge(release Release) {
	for _, section := range release.Sections {
		index := int(section.Type)
		if merged.sections[index] == nil {
			merged.sections[index] = make([]MergedItem, 0)
		}
		for _, item := range section.Items {
			merged.sections[index] = append(merged.sections[index], MergedItem{
				Release: release,
				Item:    &item,
			})
		}
	}
}

func (merged *MergedSections) Render(changelog *Changelog, doPrefix bool) string {
	sections := make([]string, int(LastChangeType())+1)
	for _, changeType := range KnownChangeTypes() {
		items := merged.sections[int(changeType)]
		if items == nil {
			continue
		}

		renderedItems := make([]string, len(items))
		for i, item := range items {
			prefix := ""
			if doPrefix {
				prefix = fmt.Sprintf("[%s] ", item.Release.Version)
			}
			renderedItems[i] = "- " + prefix + changelog.ContentWithin(&item.Item.Bounds) + "\n"
		}

		sections[int(changeType)] = "## " + ChangeTypeLabel(changeType) + "\n" + strings.Join(renderedItems, "\n")
	}

	return strings.Join(sections, "\n\n")
}
