//! Browser-only tests, ported from the Kotlin `BrowserTest` (which ran on
//! Karma + headless Chrome). Run with `cargo test --target
//! wasm32-unknown-unknown`; the configuration macro below makes the harness
//! launch a headless browser (located through `CHROMEDRIVER`) instead of
//! Node.js, because `document.cookie` only exists in a browser.

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::Window;

use super::{read_cookie, read_settings, write_cookie, write_settings};
use crate::settings::Settings;

wasm_bindgen_test_configure!(run_in_browser);

fn window() -> Window {
    web_sys::window().expect("browser tests require a `window`")
}

#[wasm_bindgen_test]
fn read_and_write_cookie() {
    write_cookie(&window(), "test", "value", 1).expect("cookie write");
    assert_eq!(Some("value".to_string()), read_cookie(&window(), "test"));
}

#[wasm_bindgen_test]
fn read_and_write_settings() {
    let settings = Settings {
        default_bang: "blah".to_string(),
        bang_chars: "!@/".to_string(),
        safe: false,
        browser_id: None,
    };
    write_settings(&window(), &settings).expect("settings write");
    assert_eq!(Some(settings), read_settings(&window()));
}
