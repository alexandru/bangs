//! The home page: wires the search form, the settings form, and the build
//! info footer.

use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Document, Event, HtmlInputElement, HtmlSpanElement, Window};

use crate::browser::{read_settings, write_settings};
use crate::build_info::BUILD_GIT_COMMIT_SHA;
use crate::settings::Settings;
use crate::url_codec;

/// Wires all home-page controls once the DOM is ready.
pub fn init_home_page(window: &Window) {
    init_search_form(window);
    init_settings_form(window);
    init_build_info(window);
}

/// Submits the search box to `/search/#q=<query>`.
fn init_search_form(window: &Window) {
    let Some(document) = window.document() else {
        return;
    };
    let Some(input): Option<HtmlInputElement> = element_by_id(&document, "search-input") else {
        return;
    };
    let Some(form) = document.get_element_by_id("search-form") else {
        return;
    };

    let on_submit: Closure<dyn FnMut(Event)> = {
        let window = window.clone();
        Closure::wrap(Box::new(move |event: Event| {
            // Only the guard is trimmed; the URL keeps the raw value, like
            // the Kotlin handler did.
            if !input.value().trim().is_empty() {
                event.prevent_default();
                let href = format!(
                    "/search/#q={}",
                    url_codec::encode_uri_component(&input.value())
                );
                if let Err(error) = window.location().set_href(&href) {
                    web_sys::console::error_1(&error);
                }
            }
        }) as Box<dyn FnMut(Event)>)
    };
    if let Err(error) =
        form.add_event_listener_with_callback("submit", on_submit.as_ref().unchecked_ref())
    {
        web_sys::console::error_1(&error);
    }
    // Page-lifetime listener: forgetting the closure keeps it registered,
    // matching the plain JS listeners of the original app.
    on_submit.forget();
}

/// Prefills the settings inputs from the cookie and persists the form on
/// every change.
fn init_settings_form(window: &Window) {
    let Some(document) = window.document() else {
        return;
    };
    let settings = read_settings(window).unwrap_or_else(Settings::default);

    let text_fields = [
        ("default-bang", settings.default_bang.clone()),
        ("bang-chars", settings.bang_chars.clone()),
        (
            "browser-id",
            settings.browser_id.clone().unwrap_or_default(),
        ),
    ];
    for (id, value) in text_fields {
        let Some(input): Option<HtmlInputElement> = element_by_id(&document, id) else {
            continue;
        };
        input.set_value(&value);
        persist_on_change(window, &input, "input");
    }
    let Some(checkbox): Option<HtmlInputElement> = element_by_id(&document, "safe-search") else {
        return;
    };
    checkbox.set_checked(settings.safe);
    persist_on_change(window, &checkbox, "change");
}

/// Shows the build's git commit SHA in the footer.
fn init_build_info(window: &Window) {
    let Some(document) = window.document() else {
        return;
    };
    let Some(span): Option<HtmlSpanElement> = element_by_id(&document, "build-info") else {
        return;
    };
    span.set_text_content(Some(BUILD_GIT_COMMIT_SHA));
}

/// Registers a listener that re-reads the whole settings form and persists
/// it, mirroring the Kotlin `onSettingsChanged`.
fn persist_on_change(window: &Window, element: &HtmlInputElement, event: &str) {
    let on_change: Closure<dyn FnMut(Event)> = {
        let window = window.clone();
        Closure::wrap(
            Box::new(move |_: Event| persist_current_settings(&window)) as Box<dyn FnMut(Event)>
        )
    };
    if let Err(error) =
        element.add_event_listener_with_callback(event, on_change.as_ref().unchecked_ref())
    {
        web_sys::console::error_1(&error);
    }
    on_change.forget(); // page-lifetime listener
}

/// Reads the current form values into [`Settings`] and writes the cookie.
fn persist_current_settings(window: &Window) {
    let Some(document) = window.document() else {
        return;
    };
    let defaults = Settings::default();
    let settings = Settings {
        // Missing elements fall back to defaults; present-but-empty values
        // are kept as-is (an empty default bang reads back as "unset").
        default_bang: text_value(&document, "default-bang")
            .unwrap_or_else(|| defaults.default_bang.clone()),
        bang_chars: text_value(&document, "bang-chars")
            .unwrap_or_else(|| defaults.bang_chars.clone()),
        safe: element_by_id::<HtmlInputElement>(&document, "safe-search")
            .map(|checkbox| checkbox.checked())
            .unwrap_or(defaults.safe),
        browser_id: text_value(&document, "browser-id").filter(|value| !value.is_empty()),
    };
    if let Err(error) = write_settings(window, &settings) {
        web_sys::console::error_1(&error);
    }
}

/// Returns the element with `id` downcast to `T`, or `None` when absent or of
/// a different type (the Kotlin code used unchecked casts instead).
fn element_by_id<T: JsCast>(document: &Document, id: &str) -> Option<T> {
    document.get_element_by_id(id)?.dyn_into::<T>().ok()
}

/// Current value of a text input; `None` when the element is absent.
fn text_value(document: &Document, id: &str) -> Option<String> {
    let input: HtmlInputElement = element_by_id(document, id)?;
    Some(input.value())
}
