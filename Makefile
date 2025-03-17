.PHONY: dist gen-bangs clean

compile:
	./gradlew compileTestKotlinWasmJs

#gen-bangs:
#	./scripts/gen-bangs.js > src/wasmJsMain/kotlin/data.kt

dist:
	./gradlew wasmJsBrowserDistribution

clean:
	./gradlew clean

run-dev:
	./gradlew wasmJsBrowserRun -t

run-prod:
	./gradlew wasmJsBrowserProductionRun
