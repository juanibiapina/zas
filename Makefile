default: test_integration

build:
	cargo build

test_unit:
	cargo test

test_integration: build test_unit
	bats integration

watch:
	git ls-files | entr -c make

.PHONY: test_unit test_integration build default watch
