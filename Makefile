default: test_unit

build:
	cargo build

test_unit:
	cargo test

test_integration: build
	bats integration

.PHONY: test_unit test_integration build default
