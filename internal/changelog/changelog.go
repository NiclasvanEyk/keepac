package changelog

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
	HeadlineBounds Bounds
	Bounds         Bounds
	Sections       []Section
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

	HeadlineBounds Bounds
	Bounds         Bounds
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

func KnownChangeTypes() []ChangeType {
	return []ChangeType{
		Added,
		Changed,
		Deprecated,
		Fixed,
		Removed,
		Security,
	}
}

func LastChangeType() ChangeType {
	return Security
}

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

func EmptyBounds() Bounds {
	return Bounds{Start: -1, Stop: -1}
}

type Section struct {
	Type   ChangeType
	Items  []Item
	Bounds Bounds
}

type Item struct {
	Bounds Bounds
}

func HasChanges(sections *([]Section)) bool {
	for _, section := range *sections {
		for range section.Items {
			return true
		}
	}

	return false
}

func (changelog *Changelog) FindRelease(version string) *Release {
	for _, release := range changelog.Releases.Past {
		if release.Version == version {
			return &release
		}
	}

	return nil
}
