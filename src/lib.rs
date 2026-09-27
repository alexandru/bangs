//! Bangs: a personal search interface with DuckDuckGo-style "!bang" shortcuts.
//!
//! The whole app is compiled to WebAssembly and served as a static site:
//!
//! - `/` shows the home page (a search box plus settings kept in a cookie);
//! - `/search/` reads the `q` parameter, resolves the bangs in it against the
//!   engine registry in [`settings`], and redirects the browser.
//!
//! [`models`] defines the data shapes, [`settings`] the engine registry,
//! and [`utils`] the pure logic. [`app`] is the page-level logic and
//! [`browser`] the DOM glue; both compile only for wasm, since they
//! cannot run outside a browser.

pub mod build_info;
pub mod models;
pub mod settings;
pub mod utils;

#[cfg(target_arch = "wasm32")]
pub mod app;
#[cfg(target_arch = "wasm32")]
pub mod browser;
