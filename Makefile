.PHONY: build dist clean test

build:
	wasm-pack build --target web --release

dist: build
	mkdir -p dist
	cp -r pkg dist/
	cp index.html dist/
	cp -r search dist/
	cp -r src/jsMain/resources/assets dist/ 2>/dev/null || true
	cp src/jsMain/resources/favicon.ico dist/ 2>/dev/null || true
	cp src/jsMain/resources/search.xml dist/ 2>/dev/null || true

clean:
	cargo clean
	rm -rf pkg dist

test:
	cargo test

run-dev:
	python3 -m http.server 8080

.DEFAULT_GOAL := build
