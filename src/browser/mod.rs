//! DOM glue: everything that touches the browser. Pure logic lives in the
//! sibling modules and receives DOM values as arguments; this module reads
//! and writes them.

mod home;
mod search_page;

#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Event, HtmlDocument, Window};

use crate::cookie;
use crate::settings::Settings;
use crate::url_codec;

/// Entry point the generated JS bootstrap (`static/main.js`) calls once the
/// wasm module is instantiated. Runs the search flow on `/search/`, and
/// wires the home page otherwise.
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let Some(window) = web_sys::window() else {
        return Err(JsValue::from_str("no global `window` available"));
    };
    let on_search_page = window
        .location()
        .pathname()
        .map(|path| path.starts_with("/search/"))
        .unwrap_or(false);
    if on_search_page {
        return search_page::trigger_search(&window);
    }
    // The wasm module may finish loading after the DOM has been parsed, in
    // which case `DOMContentLoaded` would never fire; only wait while the
    // document is still loading.
    let ready = window
        .document()
        .map(|document| document.ready_state() != "loading")
        .unwrap_or(false);
    if ready {
        home::init_home_page(&window);
        return Ok(());
    }
    let Some(document) = window.document() else {
        return Err(JsValue::from_str("no `document` available"));
    };
    let on_ready: Closure<dyn FnMut(Event)> = {
        let window = window.clone();
        Closure::wrap(
            Box::new(move |_: Event| home::init_home_page(&window)) as Box<dyn FnMut(Event)>
        )
    };
    let listener: &js_sys::Function = on_ready.as_ref().unchecked_ref();
    document.add_event_listener_with_callback("DOMContentLoaded", listener)?;
    // Page-lifetime listener: forgetting the closure keeps it registered,
    // matching the plain JS listeners of the original app.
    on_ready.forget();
    Ok(())
}

/// Reads a cookie value through `document.cookie`, percent-decoded;
/// `None` when absent, empty, or unreadable.
pub fn read_cookie(window: &Window, name: &str) -> Option<String> {
    let document: HtmlDocument = window.document()?.dyn_into().ok()?;
    let cookie_header = document.cookie().ok()?;
    let raw = cookie::find_cookie_value(&cookie_header, name)?;
    url_codec::decode_uri_component(raw)
}

/// Writes a cookie like the original `writeCookie`: percent-encoded value,
/// expiry computed from now in days (formatted with JS `Date#toUTCString`),
/// `path=/`, and `domain=<current hostname>`.
pub fn write_cookie(
    window: &Window,
    name: &str,
    value: &str,
    days_until_expire: u32,
) -> Result<(), JsValue> {
    let encoded_value = url_codec::encode_uri_component(value);
    let expires_ms = js_sys::Date::now() + f64::from(days_until_expire) * 86_400_000.0;
    let expires = String::from(js_sys::Date::new(&JsValue::from_f64(expires_ms)).to_utc_string());
    let hostname = window.location().hostname()?;
    let cookie = cookie::build_set_cookie(name, &encoded_value, &expires, &hostname);
    let document: HtmlDocument = window
        .document()
        .ok_or_else(|| JsValue::from_str("no `document` available"))?
        .dyn_into()
        .map_err(|_| JsValue::from_str("document is not an `HtmlDocument`"))?;
    document.set_cookie(&cookie)
}

/// Reads a query parameter, from `location.search` first and then
/// `location.hash`, decoded and trimmed like the Kotlin `getQueryParameter`.
pub fn get_query_parameter(window: &Window, name: &str) -> Result<Option<String>, JsValue> {
    let location = window.location();
    let search = location.search()?;
    let hash = location.hash()?;
    let raw = url_codec::find_query_param(&search, name)
        .or_else(|| url_codec::find_query_param(&hash, name));
    Ok(raw.and_then(url_codec::decode_query_value))
}

/// Reads and parses the `settings` cookie; `None` when absent or malformed.
pub fn read_settings(window: &Window) -> Option<Settings> {
    let json = read_cookie(window, "settings")?;
    web_sys::console::log_1(&JsValue::from_str(&format!("Restoring settings: {json}")));
    Settings::from_json(&json)
}

/// Persists `settings` in the `settings` cookie for ten years.
pub fn write_settings(window: &Window, settings: &Settings) -> Result<(), JsValue> {
    let json = settings.to_json();
    web_sys::console::log_1(&JsValue::from_str(&format!("Saving settings: {json}")));
    write_cookie(window, "settings", &json, 365 * 10)
}

/// Navigates the tab to `url`. With `debug`, aborts with a JS error carrying
/// the redirect target instead — the original threw an exception with the
/// same message, which `static/main.js` reports through `console.error`.
pub fn redirect_to_url(window: &Window, url: &str, debug: bool) -> Result<(), JsValue> {
    if debug {
        return Err(JsValue::from_str(&format!("Redirect to {url}")));
    }
    window.location().replace(url)?;
    Ok(())
}
