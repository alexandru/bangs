# Rust/WASM build for bangs.
#
# `make dist` assembles the deployable static site in dist/: the wasm bundle
# goes into a directory named after the git commit (cache busting), and the
# HTML templates get that path substituted for __BANGS_BUILD_TAG__.

TARGET := wasm32-unknown-unknown
DIST := dist
# First 7 characters of HEAD's SHA; also embedded in the binary by build.rs.
BUILD_TAG := $(shell git rev-parse HEAD 2>/dev/null | cut -c1-7)
ifeq ($(strip $(BUILD_TAG)),)
BUILD_TAG := unknown
endif
# Must match the `wasm-bindgen` version pinned in Cargo.toml exactly.
BINDGEN_VERSION := 0.2.129

.PHONY: all build dist test test-wasm install-bindgen serve clean

all: dist

build:
	cargo build --release --target $(TARGET)

dist: build
	mkdir -p $(DIST)/bangs-$(BUILD_TAG) $(DIST)/search $(DIST)/assets
	wasm-bindgen --target web --no-typescript \
		--out-dir $(DIST)/bangs-$(BUILD_TAG) \
		target/$(TARGET)/release/bangs.wasm
	cp static/main.js $(DIST)/bangs-$(BUILD_TAG)/main.js
	cp static/index.html $(DIST)/index.html
	cp static/search/index.html $(DIST)/search/index.html
	cp static/search.xml static/favicon.ico $(DIST)/
	cp static/assets/search.svg $(DIST)/assets/
	sed -i 's/__BANGS_BUILD_TAG__/bangs-$(BUILD_TAG)/g' \
		$(DIST)/index.html $(DIST)/search/index.html

test:
	cargo test

test-wasm:
	cargo test --target $(TARGET)

install-bindgen:
	cargo install wasm-bindgen-cli --locked --version $(BINDGEN_VERSION)

serve: dist
	python3 -m http.server 8080 --directory $(DIST)

clean:
	cargo clean
	rm -rf $(DIST)
