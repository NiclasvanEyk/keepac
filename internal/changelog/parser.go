package changelog

import (
	"fmt"
	"math"
	"regexp"
	"strings"

	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/ast"
	"github.com/yuin/goldmark/text"
)

func Parse(source []byte) Changelog {
	reader := text.NewReader(source)
	parser := goldmark.DefaultParser()
	root := parser.Parse(reader)

	run := parserRun{
		title:                       "",
		currentRelease:              nil,
		currentReleaseIsNextRelease: false,
		nextRelease:                 nil,
		source:                      source,
	}

	err := ast.Walk(root, func(node ast.Node, entering bool) (ast.WalkStatus, error) {
		if !entering {
			return ast.WalkContinue, nil
		}

		if node.Kind() == ast.KindHeading {
			heading := node.(*ast.Heading)

			if run.title == "" && heading.Level == 1 {
				run.handleTitle(heading)
			}

			if heading.Level == 2 {
				run.handleNewRelease(heading)
			}

			if heading.Level == 3 {
				run.handleNewReleaseSection(heading)
			}
		}

		// The following operations require a section of a release to be present
		if run.currentSection() == nil {
			return ast.WalkContinue, nil
		}

		if node.Kind() == ast.KindList {
			run.handleListOfChanges(node.(*ast.List))
		}

		if node.Kind() == ast.KindListItem {
			run.handleListOfChangesItem(node.(*ast.ListItem))
		}

		return ast.WalkContinue, nil
	})
	if err != nil {
		fmt.Printf("Warning: %s\n", err.Error())
	}

	run.finalizeCurrentRelease(len(source))

	changelog := Changelog{
		Title:  run.title,
		source: string(source),
		Releases: Releases{
			Next: run.nextRelease,
			Past: run.releases,
		},
	}

	return changelog
}

type parserRun struct {
	source                      []byte
	title                       string
	releases                    []Release
	currentRelease              *Release
	currentReleaseIsNextRelease bool
	nextRelease                 *NextRelease
}

func (run *parserRun) currentSection() *Section {
	if run.currentRelease == nil {
		return nil
	}

	if len(run.currentRelease.Sections) < 1 {
		return nil
	}

	return &run.currentRelease.Sections[len(run.currentRelease.Sections)-1]
}

func (run *parserRun) finalizeCurrentRelease(stop int) {
	if run.currentRelease == nil {
		return
	}

	if run.currentReleaseIsNextRelease {
		run.finalizeCurrentAsNextRelease(stop)
	} else {
		run.currentRelease.Bounds.Stop = stop
		run.finalizeCurrentAsPastRelease()
	}
}

func computeBounds(node ast.Node) Bounds {
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

func (run *parserRun) finalizeCurrentAsNextRelease(stop int) {
	run.nextRelease = &NextRelease{
		Bounds: Bounds{
			Start: run.currentRelease.HeadlineBounds.Start - 3,
			Stop:  stop,
		},
		HeadlineBounds: run.currentRelease.HeadlineBounds,
		Sections:       run.currentRelease.Sections,
	}
	run.currentRelease = nil
}

func (run *parserRun) finalizeCurrentAsPastRelease() {
	run.releases = append(run.releases, *run.currentRelease)
	run.currentRelease = nil
}

// Sets the title
func (run *parserRun) handleTitle(heading *ast.Heading) {
	run.title = string(heading.Text(run.source))
}

// Finalizes the current release and prepares a new one
func (run *parserRun) handleNewRelease(heading *ast.Heading) {
	headingBounds := computeBounds(heading)

	// Close current release if necessary
	run.finalizeCurrentRelease(headingBounds.Start - 3)

	// Prepare a new one
	line := string(heading.Text(run.source))
	r := NewRelease(parseVersion(line), parseDate(line))
	r.Yanked = strings.Contains(line, "[YANKED]")
	r.HeadlineBounds = headingBounds
	r.Bounds = Bounds{
		// we subtract the length of "## " to achieve better insertion points
		Start: r.HeadlineBounds.Start - 3,
		Stop:  r.HeadlineBounds.Stop, // This will be incremented later
	}

	// Set the prepared one as active
	run.currentRelease = &r
	run.currentReleaseIsNextRelease = line == "[Unreleased]" || line == "Unreleased"
}

// Finalizes the current section and prepares a new one
func (run *parserRun) handleNewReleaseSection(heading *ast.Heading) {
	bounds := computeBounds(heading)
	bounds.Start = bounds.Start - 4 // we subtract the length of "### " to achieve better insertion points
	section := Section{
		Type:   ParseChangeType(string(heading.Text(run.source))),
		Items:  make([]Item, 0),
		Bounds: bounds,
	}
	run.currentRelease.Sections = append(run.currentRelease.Sections, section)
}

// Can be used to widen the bounds of a section
func (run *parserRun) handleListOfChanges(list *ast.List) {
	bounds := computeBounds(list)
	run.currentSection().Bounds.Stop = bounds.Stop
}

func (run *parserRun) handleListOfChangesItem(item *ast.ListItem) {
	bounds := computeBounds(item)

	currentSection := run.currentSection()
	currentSection.Bounds.Stop = bounds.Stop
	currentSection.Items = append(currentSection.Items, Item{Bounds: bounds})
}

func parseVersion(line string) string {
	semver := regexp.MustCompile(`(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?`)
	slice := semver.Find([]byte(line))

	if slice != nil {
		return string(slice)
	}

	return ""
}

func parseDate(line string) string {
	semver := regexp.MustCompile("[0-9]{4}-[0-9]-{2}-[0-9]{2}")
	slice := semver.Find([]byte(line))

	if slice != nil {
		return string(slice)
	}

	return ""
}
