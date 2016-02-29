# config

VERSION="v0.15.0"
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
	mkdir -p tmp/zas/libexec
	cp target/release/zasd tmp/zas/bin
	cp -r resources tmp/zas
	cp -r libexec tmp/zas
	( cd tmp/zas/bin && ln -s ../libexec/zas )
	mv tmp/zas tmp/zas-${VERSION}+${PLATFORM}
	cd tmp && tar -czf zas-${VERSION}+${PLATFORM}.tar.gz zas-${VERSION}+${PLATFORM}

.PHONY: release
release: clean_tmp test_integration tar

# other

.PHONY: clean_tmp
clean_tmp:
	rm -rf tmp

.PHONY: clean
clean: clean_tmp
	cargo clean
