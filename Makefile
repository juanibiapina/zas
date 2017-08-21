.PHONY: default
default: test_integration

.PHONY: build_debug
build_debug:
	cargo build

.PHONY: test_unit
test_unit:
	cargo test

.PHONY: test_integration
test_integration: build_debug test_unit
	./run_tests.sh

.PHONY: clean
clean:
	cargo clean
