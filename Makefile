.PHONY: default
default: test_integration

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: test_unit
test_unit:
	cargo test

.PHONY: test_integration
test_integration: build test_unit
	bats integration

.PHONY: watch
watch:
	git ls-files -c -m | entr -c make
