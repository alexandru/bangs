# Rust/WASM build for bangs.
#
# `make dist` assembles the deployable static site in dist/: the wasm bundle
# goes into a directory named after the git commit (cache busting), and the
# HTML templates get that path substituted for __BANGS_BUILD_TAG__.
#
# The build provisions its own prerequisites: the wasm32 standard library is
# installed via rustup when missing (also declared in rust-toolchain.toml),
# the wasm-bindgen CLI is installed at the pinned version when missing or
# outdated, and wasm-opt is downloaded from the official binaryen release
# (same prebuilts wasm-pack uses). No manual setup beyond a working Rust
# installation.

TARGET := wasm32-unknown-unknown
DIST := dist
# First 7 characters of HEAD's SHA; also embedded in the binary by build.rs.
BUILD_TAG := $(shell git rev-parse HEAD 2>/dev/null | cut -c1-7)
ifeq ($(strip $(BUILD_TAG)),)
BUILD_TAG := unknown
endif
# Must match the `wasm-bindgen` version pinned in Cargo.toml exactly.
BINDGEN_VERSION := 0.2.129
# Binaryen release to pull wasm-opt prebuilts from (wasm-pack pins 130 too).
BINARYEN_VERSION := 130
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)
ifeq ($(UNAME_S),Linux)
BINARYEN_TARGET := $(if $(filter aarch64,$(UNAME_M)),aarch64-linux,x86_64-linux)
endif
ifeq ($(UNAME_S),Darwin)
BINARYEN_TARGET := $(if $(filter arm64,$(UNAME_M)),arm64-macos,x86_64-macos)
endif
# Resolved when the Makefile is parsed; falls back to cargo's install
# location ($CARGO_HOME/bin, default ~/.cargo/bin) so `make dist` works even
# when that directory is not on PATH (ensure-bindgen installs there).
WASM_BINDGEN := $(shell command -v wasm-bindgen 2>/dev/null || echo "$${CARGO_HOME:-$$HOME/.cargo}/bin/wasm-bindgen")
# Downloaded once into the user cache and reused afterwards. The tarball's
# top-level directory is binaryen-version_<v>/ on every platform.
WASM_OPT := $(HOME)/.cache/bangs/binaryen-version_$(BINARYEN_VERSION)/bin/wasm-opt

.PHONY: all build dist test test-wasm ensure-wasm-target ensure-bindgen ensure-wasm-opt install-bindgen serve clean

all: dist

# Installs the wasm32 standard library when rustup manages the toolchain;
# skipped silently otherwise (the cargo build then fails with rustc's own
# hint, since targets cannot be added to a rustup-less toolchain).
ensure-wasm-target:
	@if command -v rustup >/dev/null 2>&1; then rustup target add $(TARGET); fi

build: ensure-wasm-target
	cargo build --release --target $(TARGET)

# Installs the wasm-bindgen CLI when it is missing or its version differs
# from the pinned one (the CLI and the crate must match exactly).
ensure-bindgen:
	@if ! $(WASM_BINDGEN) --version 2>/dev/null | grep -q "$(BINDGEN_VERSION)"; then \
		$(MAKE) install-bindgen; \
	fi

# Downloads the official wasm-opt prebuilt once, into the user cache.
ensure-wasm-opt:
	@if [ -z "$(BINARYEN_TARGET)" ]; then \
		echo "error: no binaryen release target for $(UNAME_S) $(UNAME_M)"; \
		exit 1; \
	elif [ ! -x "$(WASM_OPT)" ]; then \
		mkdir -p $(HOME)/.cache/bangs; \
		curl -fsSL https://github.com/WebAssembly/binaryen/releases/download/version_$(BINARYEN_VERSION)/binaryen-version_$(BINARYEN_VERSION)-$(BINARYEN_TARGET).tar.gz \
			| tar -xz -C $(HOME)/.cache/bangs; \
	fi

dist: build ensure-bindgen ensure-wasm-opt
	mkdir -p $(DIST)/bangs-$(BUILD_TAG) $(DIST)/search $(DIST)/assets
	$(WASM_BINDGEN) --target web --no-typescript \
		--out-dir $(DIST)/bangs-$(BUILD_TAG) \
		target/$(TARGET)/release/bangs.wasm
	$(WASM_OPT) -Oz --enable-bulk-memory \
		$(DIST)/bangs-$(BUILD_TAG)/bangs_bg.wasm \
		-o $(DIST)/bangs-$(BUILD_TAG)/bangs_bg.opt.wasm
	mv $(DIST)/bangs-$(BUILD_TAG)/bangs_bg.opt.wasm $(DIST)/bangs-$(BUILD_TAG)/bangs_bg.wasm
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
