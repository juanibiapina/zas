# default

.PHONY: default
default: test_integration

# development

.PHONY: build_debug
build_debug:
	cargo build

.PHONY: test_unit
test_unit:
	cargo test

.PHONY: test_integration
test_integration: build_debug test_unit
	bats integration

.PHONY: watch
watch:
	git ls-files -c -m | entr -c make

# release

.PHONY: build_release
build_release:
	cargo build --release

