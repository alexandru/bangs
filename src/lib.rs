//! Bangs: a personal search interface with DuckDuckGo-style "!bang" shortcuts.
//!
//! The whole app is compiled to WebAssembly and served as a static site:
//!
//! - `/` shows the home page (a search box plus settings kept in a cookie);
//! - `/search/` reads the `q` parameter, resolves the bangs in it against
//!   the engine registry in [`bangs`], and redirects the browser.
//!
//! Modules are split so that everything testable is pure Rust ([`bangs`],
//! [`search`], [`settings`], [`cookie`], [`url_codec`]), while [`browser`]
//! holds the DOM glue that cannot run outside a browser.

pub mod bangs;
pub mod build_info;
pub mod cookie;
pub mod search;
pub mod settings;
pub mod url_codec;

#[cfg(target_arch = "wasm32")]
pub mod browser;
