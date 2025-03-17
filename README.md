# Bangs

## Development

For running the application in development-mode:

```bash
./gradlew wasmJsBrowserRun -t
```

To build a production-ready distribution (available after the build in 
`./build/dist/wasmJs/productionExecutable`):

```bash
./gradlew wasmJsBrowserDistribution
```

To run the application in the browser using the production-ready distribution:

```bash
./gradlew wasmJsBrowserProductionRun
```
