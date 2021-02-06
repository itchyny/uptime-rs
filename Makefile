.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: install
install:
	cargo install --path .

.PHONY: test
test: build
	cargo test

.PHONY: clean
clean:
	cargo clean

VERSION := $$(sed -n '/^version =/s/^.*  *"\([0-9][0-9.]*\)".*$$/\1/p' Cargo.toml)
GIT_DIFF := $$(git diff --name-only)

.PHONY: bump
bump:
ifneq ($(shell git status --porcelain),)
	$(error git workspace is dirty)
endif
ifneq ($(shell git rev-parse --abbrev-ref HEAD),master)
	$(error current branch is not master)
endif
	@printf "Bump up version in Cargo.toml. Press Enter to proceed: "
	@read -n1
	@[ "$(GIT_DIFF)" == "Cargo.toml" ] || { \
		echo "Version is not updated or unrelated file is updated:"; \
		[ -z "$(GIT_DIFF)" ] || printf "  %s\n" $(GIT_DIFF); \
		exit 1; \
	}
	git commit -am "bump up version to $(VERSION)"
	git tag "v$(VERSION)"
	git push origin master
	git push origin "refs/tags/v$(VERSION)"

.PHONY: publish
publish:
	cargo publish
