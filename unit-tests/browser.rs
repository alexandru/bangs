//! Browser-only tests, ported from the Kotlin `BrowserTest` (which ran on
//! Karma + headless Chrome). Run with `cargo test --target
//! wasm32-unknown-unknown`; the configuration macro below makes the harness
//! launch a headless browser (located through `CHROMEDRIVER`) instead of
//! Node.js, because `document.cookie` only exists in a browser.

use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::Window;

use super::{read_cookie, read_settings, write_cookie, write_settings};
use crate::settings::Settings;

wasm_bindgen_test_configure!(run_in_browser);

fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("browser tests require a `window`"))
}

#[wasm_bindgen_test]
fn read_and_write_cookie() -> Result<(), JsValue> {
    let window = window()?;
    write_cookie(&window, "test", "value", 1)?;
    assert_eq!(Some("value".to_string()), read_cookie(&window, "test"));
    Ok(())
}

#[wasm_bindgen_test]
fn read_and_write_settings() -> Result<(), JsValue> {
    let settings = Settings {
        default_bang: "blah".to_string(),
        bang_chars: "!@/".to_string(),
        safe: false,
        browser_id: None,
    };
    let window = window()?;
    write_settings(&window, &settings)?;
    assert_eq!(Some(settings), read_settings(&window));
    Ok(())
}

#[wasm_bindgen_test]
fn malformed_settings_cookie_yields_no_settings() -> Result<(), JsValue> {
    let window = window()?;
    write_cookie(&window, "settings", "{not json", 1)?;
    assert_eq!(None, read_settings(&window));
    Ok(())
}
