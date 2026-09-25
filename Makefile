# Rust/WASM build for bangs.
#
# `make dist` assembles the deployable static site in dist/: the wasm bundle
# goes into a directory named after the git commit (cache busting), and the
# HTML templates get that path substituted for __BANGS_BUILD_TAG__.
#
# The build provisions its own prerequisites: the wasm32 standard library is
# installed via rustup when missing (also declared in rust-toolchain.toml),
# and the wasm-bindgen CLI is installed at the pinned version when missing or
# outdated. No manual setup is needed beyond a working Rust installation.

TARGET := wasm32-unknown-unknown
DIST := dist
# First 7 characters of HEAD's SHA; also embedded in the binary by build.rs.
BUILD_TAG := $(shell git rev-parse HEAD 2>/dev/null | cut -c1-7)
ifeq ($(strip $(BUILD_TAG)),)
BUILD_TAG := unknown
endif
# Must match the `wasm-bindgen` version pinned in Cargo.toml exactly.
BINDGEN_VERSION := 0.2.129

.PHONY: all build dist test test-wasm ensure-wasm-target ensure-bindgen install-bindgen serve clean

all: dist

# Installs the wasm32 standard library when rustup manages the toolchain;
# skipped silently otherwise (the cargo build then fails with rustc's own
# hint, since targets cannot be added to a rustup-less toolchain).
ensure-wasm-target:
	@if command -v rustup >/dev/null 2>&1; then rustup target add $(TARGET); fi

build: ensure-wasm-target
	cargo build --release --target $(TARGET)

# Installs the wasm-bindgen CLI when missing, or when the installed version
# differs from the pinned one (the CLI and the crate must match exactly).
ensure-bindgen:
	@if ! command -v wasm-bindgen >/dev/null 2>&1; then \
		$(MAKE) install-bindgen; \
	elif ! wasm-bindgen --version | grep -q "$(BINDGEN_VERSION)"; then \
		$(MAKE) install-bindgen; \
	fi

dist: build ensure-bindgen
	mkdir -p $(DIST)/bangs-$(BUILD_TAG) $(DIST)/search $(DIST)/assets
	wasm-bindgen --target web --no-typescript \
		--out-dir $(DIST)/bangs-$(BUILD_TAG) \
		target/$(TARGET)/release/bangs.wasm
	cp static/main.js $(DIST)/bangs-$(BUILD_TAG)/main.js
	cp static/index.html $(DIST)/index.html
	cp static/search/index.html $(DIST)/search/index.html
	cp static/search.xml static/favicon.ico $(DIST)/
	cp static/assets/search.svg $(DIST)/assets/
	sed -i.bak 's/__BANGS_BUILD_TAG__/bangs-$(BUILD_TAG)/g' \
		$(DIST)/index.html $(DIST)/search/index.html
	rm -f $(DIST)/index.html.bak $(DIST)/search/index.html.bak

test:
	cargo test

test-wasm: ensure-wasm-target
	cargo test --target $(TARGET)

install-bindgen:
	cargo install wasm-bindgen-cli --locked --force --version $(BINDGEN_VERSION)

serve: dist
	python3 -m http.server 8080 --directory $(DIST)

clean:
	cargo clean
	rm -rf $(DIST)
