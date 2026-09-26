//! The `/search/` page: resolves the `q` parameter to a redirect.

use wasm_bindgen::JsValue;
use web_sys::Window;

use crate::browser::{get_query_parameter, read_settings, redirect_to_url};
use crate::search;
use crate::settings::Settings;

/// Reads the query, resolves it against the engine registry, and redirects.
pub fn trigger_search(window: &Window) -> Result<(), JsValue> {
    let stored = read_settings(window).unwrap_or_default();
    let settings = override_settings_from_url(window, &stored)?;
    let debug = get_query_parameter(window, "debug")?.is_some();
    let Some(raw_query) = get_query_parameter(window, "q")? else {
        // Empty or missing query: go home.
        return redirect_to_origin(window, debug);
    };
    match search::resolve_target_url(&raw_query, &settings) {
        Some(url) => redirect_to_url(window, &url, debug),
        // Unreachable while the registry keeps the fallback default bang.
        None => redirect_to_origin(window, debug),
    }
}

/// Applies the `browserId`, `defaultBang`, `bangChars`, and `safe` URL
/// parameters on top of the stored settings.
fn override_settings_from_url(window: &Window, settings: &Settings) -> Result<Settings, JsValue> {
    let browser_id = get_query_parameter(window, "browserId")?;
    let default_bang = get_query_parameter(window, "defaultBang")?;
    let bang_chars = get_query_parameter(window, "bangChars")?;
    let safe =
        get_query_parameter(window, "safe")?.and_then(|raw| crate::settings::parse_safe_flag(&raw));
    Ok(settings.with_url_overrides(browser_id, default_bang, bang_chars, safe))
}

fn redirect_to_origin(window: &Window, debug: bool) -> Result<(), JsValue> {
    redirect_to_url(window, &window.location().origin()?, debug)
}
