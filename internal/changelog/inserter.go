package changelog

import (
	"strings"
)

// func (changelog *Changelog) ensureNextReleaseSectionExists() *NextRelease {
// 	if changelog.Releases.Next != nil {
// 		return changelog.Releases.Next
// 	}
//
// 	releases := changelog.Releases.Past
// 	if len(releases) == 0 {
// 		// The changelog is completely empty, so we'll just insert at the end of the
// 		// file
// 		changelog.InsertAt(len(changelog.source), "\n\n## [Unreleased]\n\n\n")
//
// 		return &NextRelease{}
// 	}
//
// 	// There are past releases, but not yet a next one.
// 	changelog.InsertAt(releases[0].Begin, "## [Unreleased]\n\n\n")
//
// 	// TODO: The NextRelease also should have bounds
// 	return &NextRelease{}
// }

// We need to know a few key points for insertion:
// # Changelog
//
// ## [Next]
//
// ### Added
//
// - Something
//
// ### Removed                        <-- Also needed to prepend new sections
//
// - First
// - Second
// - Third                            <-- Most sections can be appended to, so we need this information
//
// ## [1.0.0] - 2000-01-01            <-- This is needed, so that we can prepend new sections
//
// ### Added
//
// - The initial version

func (changelog *Changelog) InsertAt(insertionPoint int, contents string) {
	source := changelog.source
	newSource := source[:insertionPoint] + contents + source[insertionPoint:]
	changelog.source = newSource

	addedContentLength := len(contents)

	nextRelease := changelog.Releases.Next

	sections := make([]*Section, 0)

	if nextRelease != nil {
		for _, section := range nextRelease.Sections {
			sections = append(sections, &section)
		}
	}

	for _, release := range changelog.Releases.Past {
		for _, section := range release.Sections {
			sections = append(sections, &section)
		}
	}

	for _, section := range sections {
		if section.Bounds.Start >= insertionPoint {
			section.Bounds.Start += addedContentLength
		}

		if section.Bounds.Stop >= insertionPoint {
			section.Bounds.Stop += addedContentLength
		}
	}
}

func (changelog *Changelog) AddItem(changeType ChangeType, contents string) string {
	parts := make([]string, 0)

	shouldCreateNextRelease := changelog.Releases.Next == nil
	if shouldCreateNextRelease {
		parts = append(parts, "## [Unreleased]")
	}

	shouldAddSection := shouldCreateNextRelease || changelog.Releases.Next.FindSection(changeType) == nil
	if shouldAddSection {
		parts = append(parts, "### "+ChangeTypeLabel(changeType))
	}

	parts = append(parts, contents)
	newContent := strings.Join(parts, "\n\n")

	insertionPoint, padding := determineInsertionPoint(changeType, changelog)

	return changelog.source[:insertionPoint] + padding.ApplyTo(newContent) + changelog.source[insertionPoint:]
}

type Padding struct {
	Before int
	After  int
}

func (p *Padding) ApplyTo(subject string) string {
	return strings.Repeat("\n", p.Before) + subject + strings.Repeat("\n", p.After)
}

// Returns the index at which to insert and the amount of Padding
func determineInsertionPoint(changeType ChangeType, changelog *Changelog) (int, Padding) {
	nextRelease := changelog.Releases.Next
	if nextRelease == nil {
		if len(changelog.Releases.Past) == 0 {
			// We have an empty changelog with just the title:
			// # Changelog                  <-- Add here
			return changelog.Stop(), Padding{Before: 2, After: 0}
		}

		// We have some releases, but no next one:
		// # Changelog
		//
		// ## [1.1.0] - 2020-01-01        <-- Add before this line
		//
		// ### Added
		return changelog.Releases.Past[0].Begin, Padding{Before: 0, After: 2}
	}

	// At this point we can be sure that we need to insert somewhere inside the
	// [Unreleased] section:
	// # Changelog
	//
	// ## [Unreleased]
	//
	// ### Added
	//
	// - Something
	// - Something else              <-- Add after this line
	//
	// ## [1.1.0] - 2020-01-01
	existingSection := nextRelease.FindSection(changeType)
	if existingSection != nil {
		return existingSection.Bounds.Start, Padding{Before: 1, After: 0}
	}

	// Now we know, that the section does not exist yet.
	//
	// To make things easy we first handle the edge case where there is an
	// [Unreleased] heading, without any actual sections in it.
	if len(nextRelease.Sections) == 0 {
		// It could be the case, that we have no past releases at all
		// # Changelog
		//
		// ## [Unreleased]               <-- Add after this line
		if len(changelog.Releases.Past) == 0 {
			return changelog.Stop(), Padding{Before: 2, After: 0}
		}

		latestRelease := changelog.Releases.Past[len(changelog.Releases.Past)-1]
		return latestRelease.Begin, Padding{Before: 0, After: 2}
	}

	// Now with all other edge cases handled we can shift our focus to adding a
	// new section in the right position. It would be nice if all of our sections
	// would follow the same order (the one mentioned by keepachangelog, which is
	// also followed in the definition of the `ChangeType` enum). This would be
	// easy if we could assume that the sections are guaranteed to be in proper
	// order, however this is a bold assumption to make given a fair share of
	// changelogs are edited by hand.

	// This is rather simple, if we can simply prepend it before an existing
	// section.
	canPrepend := int(changeType) < len(nextRelease.Sections)
	if canPrepend {
		followingSection := nextRelease.Sections[int(changeType)]
		return followingSection.Bounds.Start, Padding{Before: 0, After: 2}
	}

	// Now the only thing left is the case where we need to append the new
	// section at the very end of the [Unreleased] section. Another way of
	// framing this is inserting it before the latest release, which is
	// guaranteed to exist, since we handled this edge case earlier.
	return changelog.Releases.Past[0].Begin, Padding{Before: 0, After: 2}
}
