# Bangs

My own search interface, exposing custom "bangs" (see [DuckDuckGo](https://duckduckgo.com/bangs)). 
Inspired by [unduck](https://github.com/t3dotgg/unduck).

<https://bangs.alexn.org>

---

> [!NOTE]
> This UI supports just the list of bangs I care about, not the full list.
> Meant for my own use, not for everyone.

## Technology Stack

This project is built with:
- **Rust** compiled to **WebAssembly (WASM)** for the dynamic client-side functionality
- Lightweight and fast (~50KB gzipped total)
- Zero JavaScript framework dependencies

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack
```

### Build Commands

```bash
# Build the WASM module
make build

# Run tests
make test

# Create distribution bundle
make dist

# Start development server
make run-dev
# Then open http://localhost:8080
```

## Project Structure

```
├── src/
│   ├── lib.rs         # Main entry point and DOM initialization
│   ├── models.rs      # Data structures (Bang, Query, Referral, Settings)
│   ├── settings.rs    # Search engines and configuration
│   └── utils.rs       # Utility functions
├── index.html         # Homepage
├── search/            # Search endpoint
├── Cargo.toml         # Rust dependencies and build config
└── Makefile           # Build scripts
```

## License

Apache-2.0
