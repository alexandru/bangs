# Thin wrappers over the xtask tasks; the build logic lives in
# xtask/src/main.rs.

.PHONY: all dist build test test-wasm format serve clean

all: dist

dist:
	cargo xtask dist

build:
	cargo xtask build

test:
	cargo test

test-wasm:
	cargo xtask test-wasm

format:
	cargo xtask format

serve:
	cargo xtask serve

clean:
	cargo xtask clean
