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
- **Rust** compiled to **WebAssembly (WASM)** for client-side functionality
- Lightweight and fast (~34KB gzipped: 29.6KB WASM + 4.7KB JS)
- Zero JavaScript framework dependencies
- Idiomatic Rust code with comprehensive tests

## Features

- **Search with bangs**: Use `!w` for Wikipedia, `!gh` for GitHub, `!g` for Google, etc.
- **Customizable settings**: Configure default search engine, bang characters, and browser ID
- **Cookie-based persistence**: Settings are saved locally
- **Combined queries**: Use `gwp` for "Google + Wikipedia" (search Wikipedia via Google)
- **Browser referral support**: Optionally add browser referral codes for Firefox, Vivaldi, etc.

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

# Start development server (requires Python 3)
make run-dev
# Then open http://localhost:8080
```

### Manual Build

```bash
# Build WASM
wasm-pack build --target web --release

# The output will be in the pkg/ directory
```

## Project Structure

```
├── src/
│   ├── lib.rs         # Main entry point and DOM initialization
│   ├── models.rs      # Data structures (Bang, Query, Referral)
│   ├── settings.rs    # Search engines, queries, and configuration
│   └── utils.rs       # Utility functions (cookies, URL parsing, etc.)
├── index.html         # Homepage
├── search/            # Search endpoint directory
│   └── index.html     # Search page (triggers redirect)
├── Cargo.toml         # Rust dependencies and build configuration
├── build.rs           # Build script (generates git commit SHA)
└── Makefile           # Build automation
```

## Size Comparison

| Version | Size (gzipped) | Notes |
|---------|---------------|-------|
| Kotlin/JS (original) | ~16KB | Highly optimized Kotlin compiler output |
| Rust/WASM (this) | ~34KB | 29.6KB WASM + 4.7KB JS glue code |

The Rust/WASM version is about 2x larger but offers:
- **Type safety** at compile time
- **Better performance** (WASM is typically faster than JS)
- **More maintainable** codebase with modern Rust tooling
- **Memory safety** guarantees from Rust

## Deployment

The `dist/` directory contains everything needed for deployment:

```bash
# Build for production
make dist

# Deploy the dist/ directory to your static hosting service
# (GitHub Pages, Netlify, Vercel, etc.)
```

Files in `dist/`:
- `index.html` - Homepage
- `search/index.html` - Search endpoint
- `pkg/` - WASM module and JS glue code
- `assets/` - Static assets (favicon, icons, etc.)

## Development

To add new search engines or queries, edit `src/settings.rs`:

```rust
// Add a new special-purpose engine
Bang::new("https://example.com/search?q={{{s}}}", &["ex", "example"]),

// Add a new query combinator
Query::new("site:example.com%20{{{s}}}", &["ex"]),
```

Run tests after making changes:

```bash
cargo test
```

## License

Apache-2.0
