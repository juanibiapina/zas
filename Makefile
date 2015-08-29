# config

VERSION="v0.3.0-alpha"
PLATFORM="Darwin"

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

.PHONY: tar
tar: build_release
	rm -rf tmp/zas
	mkdir -p tmp/zas
	mkdir -p tmp/zas/bin
	cp target/release/zas tmp/zas/bin
	cp -r resources tmp/zas
	mv tmp/zas tmp/zas-${VERSION}+${PLATFORM}
	cd tmp && tar -czf zas-${VERSION}+${PLATFORM}.tar.gz zas-${VERSION}+${PLATFORM}

.PHONY: release
release: clean tar

# other

.PHONY: clean
clean:
	rm -rf tmp
