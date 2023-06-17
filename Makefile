all: binary shell-completions


binary: build-binary install-binary

build-binary:
	go build -o changelog

install-binary:
	cp changelog "${HOME}/.local/bin/changelog"


shell-completions: generate-shell-completions install-shell-completions

install-shell-completions:
	cp ./completions/changelog.zsh /usr/local/share/zsh/site-functions/_changelog

generate-shell-completions:
	./scripts/generate-completions

