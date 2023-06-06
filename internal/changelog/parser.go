package changelog

import (
	"math"

	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/ast"
	"github.com/yuin/goldmark/text"
)

type Changelog struct {
	source   string
	Title    string
	Releases Releases
}

// Returns the end index of the changelog
func (changelog *Changelog) Stop() int {
	return len(changelog.source)
}

type NextRelease struct {
	Sections []Section
}

type Releases struct {
	Next *NextRelease
	Past []Release
}

func (next *NextRelease) FindSection(changeType ChangeType) *Section {
	for _, section := range next.Sections {
		if section.Type == changeType {
			return &section
		}
	}

	return nil
}

type Release struct {
	Sections []Section
	Date     string
	Yanked   bool
	Version  string

	// Position of the first character of the release block (the heading)
	Begin int
}

func NewRelease(version string, date string) Release {
	return Release{
		Version:  version,
		Date:     date,
		Yanked:   false,
		Sections: make([]Section, 0),
	}
}

type ChangeType int64

const (
	Added ChangeType = iota
	Changed
	Deprecated
	Fixed
	Removed
	Security
	Unknown
)

func ChangeTypeLabel(changeType ChangeType) string {
	switch changeType {
	case Added:
		return "Added"
	case Changed:
		return "Changed"
	case Deprecated:
		return "Deprecated"
	case Fixed:
		return "Fixed"
	case Removed:
		return "Removed"
	case Security:
		return "Security"
	}
	return "Unknown"
}

func ParseChangeType(name string) ChangeType {
	switch name {
	case "Added":
		return Added
	case "Changed":
		return Changed
	case "Deprecated":
		return Deprecated
	case "Fixed":
		return Fixed
	case "Removed":
		return Removed
	case "Security":
		return Security
	}
	return Unknown
}

type Bounds struct {
	Start int
	Stop  int
}

type Section struct {
	Type   ChangeType
	Items  []string
	Bounds Bounds
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func ComputeBounds(node ast.Node) Bounds {
	start := math.MaxInt
	stop := -1

	current := node
	for current.Lines().Len() <= 0 {
		if !current.HasChildren() {
			panic("Can't iterate further down, something is off!")
		}
		current = current.FirstChild()
	}

	lines := current.Lines()
	for _, line := range lines.Sliced(0, lines.Len()) {
		start = min(start, line.Start)
		stop = max(stop, line.Stop)
	}

	return Bounds{Start: start, Stop: stop}
}

func EmptyBounds() Bounds {
	return Bounds{Start: -1, Stop: -1}
}

func Parse(source []byte) Changelog {
	reader := text.NewReader(source)
	parser := goldmark.DefaultParser()
	root := parser.Parse(reader)

	var releases = make([]Release, 0)

	var title = ""
	var currentRelease *Release

	ast.Walk(root, func(node ast.Node, entering bool) (ast.WalkStatus, error) {
		if !entering {
			return ast.WalkContinue, nil
		}

		if node.Kind() == ast.KindHeading {
			heading := node.(*ast.Heading)

			if title == "" && heading.Level == 1 {
				title = string(heading.Text(source))
			}

			if heading.Level == 2 {
				if currentRelease != nil {
					releases = append(releases, *currentRelease)
				}

				r := NewRelease("unknown", "unknown")
				currentRelease = &r
			}

			if heading.Level == 3 && entering {
				section := Section{
					Type:   ParseChangeType(string(heading.Text(source))),
					Items:  make([]string, 0),
					Bounds: EmptyBounds(),
				}
				(*currentRelease).Sections = append((*currentRelease).Sections, section)
			}
		}

		// We assume that the last list in the section defines its bounds which we
		// can use to add new changes later
		beginsListOfChanges := node.Kind() == ast.KindList && len(currentRelease.Sections) > 0
		if beginsListOfChanges {
			list := node.(*ast.List)
			bounds := ComputeBounds(list)
			currentRelease.Sections[len(currentRelease.Sections)-1].Bounds = bounds
		}

		if node.Kind() == ast.KindListItem && currentRelease != nil && len(currentRelease.Sections) > 0 {
			item := node.(*ast.ListItem)
			change := string(item.Text(source))
			section := &currentRelease.Sections[len(currentRelease.Sections)-1]
			section.Items = append(section.Items, change)
		}

		return ast.WalkContinue, nil
	})

	if currentRelease != nil {
		releases = append(releases, *currentRelease)
	}

	changelog := Changelog{
		Title:  title,
		source: string(source),
		Releases: Releases{
			Next: nil,
			Past: releases,
		},
	}

	return changelog
}
