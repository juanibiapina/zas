.PHONY: default
default: test_integration

.PHONY: build_debug
build_debug:
	cargo build

.PHONY: test_integration
test_integration: build_debug
	./run_tests.sh

.PHONY: clean
clean:
	cargo clean
