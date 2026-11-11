.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: install
install:
	cargo install --locked --path .

.PHONY: test
test: build
	cargo test

.PHONY: clean
clean:
	cargo clean

.PHONY: bump
bump:
	test -z "$$(git status --porcelain || echo .)"
	test "$$(git branch --show-current)" = "main"
	$(if $(filter command line,$(origin VERSION)),,$(eval VERSION := $(shell \
		cargo pkgid | sed 's/.*[#@]//' | xargs printf "Bump up version from %s to: " >&2; \
		read -r version; echo "$$version")))
	cargo set-version "$(VERSION)"
	git commit -am "bump up version to $(VERSION)"
	git tag "v$(VERSION)"
	git push --atomic origin main tag "v$(VERSION)"

.PHONY: publish
publish:
	cargo publish
