use wasm_bindgen::prelude::*;

mod models;
mod settings;
mod utils;

use models::Bang;
use settings::Settings;
use utils::*;

// Build info will be injected at compile time
const BUILD_GIT_COMMIT_SHA: &str = env!("GIT_COMMIT_SHA");

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let pathname = location.pathname().unwrap_or_default();

    if pathname.starts_with("/search/") {
        trigger_search()?;
    } else {
        let document = window.document().expect("should have a document on window");
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Err(e) = init_home_page() {
                web_sys::console::error_1(&format!("Error initializing home page: {:?}", e).into());
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("DOMContentLoaded", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    Ok(())
}

fn trigger_search() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    
    let settings = {
        let mut s = read_settings_from_cookie().unwrap_or_default();
        s = override_settings_from_url(&s);
        s
    };
    
    let debug = get_query_parameter("debug").is_some();
    let raw_query = match get_query_parameter("q") {
        Some(q) if !q.is_empty() => q,
        _ => {
            let origin = location.origin().unwrap_or_default();
            return redirect_to_url(&origin, debug);
        }
    };
    
    let bangs = extract_bangs_from_query(&raw_query, &settings);
    let mut found_bang: Option<Bang> = None;
    let mut query = raw_query.clone();
    
    for bang in bangs {
        found_bang = find_bang_url_by_key(&bang);
        if let Some(ref fb) = found_bang {
            query = remove_bang_from_query(&raw_query, &bang, fb.search_context.as_deref(), &settings);
            break;
        }
    }
    
    if found_bang.is_none() {
        found_bang = find_bang_url_by_key(&settings.default_bang)
            .or_else(|| find_bang_url_by_key(settings::DEFAULT_BANG_DEFAULT));
    }
    
    let url = if let Some(fb) = found_bang {
        let referral = find_referral(&settings, &fb.url);
        let encoded_query = encode_uri_component(&query);
        let s = if let Some(ref r) = referral {
            format!("{}&{}", encoded_query, r.referral)
        } else {
            encoded_query
        };
        fb.url.replace("{{{s}}}", &s)
    } else {
        // Fallback to Google
        format!("https://www.google.com/search?q={}", encode_uri_component(&query))
    };
    
    redirect_to_url(&url, debug)
}

fn init_home_page() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    // Initialize search form
    if let Some(form) = document.get_element_by_id("search-form") {
        let _input = document.get_element_by_id("search-input")
            .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok());
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            
            if let Some(input) = document.get_element_by_id("search-input")
                .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                if !input.value().trim().is_empty() {
                    event.prevent_default();
                    let query = encode_uri_component(&input.value());
                    let href = format!("/search/#q={}", query);
                    let _ = window.location().set_href(&href);
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // Initialize settings form
    let settings = read_settings_from_cookie().unwrap_or_default();
    
    // Setup default-bang input
    if let Some(elem) = document.get_element_by_id("default-bang")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok()) {
        elem.set_value(&settings.default_bang);
        
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let _ = on_settings_changed();
        }) as Box<dyn FnMut(_)>);
        
        elem.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // Setup bang-chars input
    if let Some(elem) = document.get_element_by_id("bang-chars")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok()) {
        elem.set_value(&settings.bang_chars);
        
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let _ = on_settings_changed();
        }) as Box<dyn FnMut(_)>);
        
        elem.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // Setup browser-id input
    if let Some(elem) = document.get_element_by_id("browser-id")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok()) {
        elem.set_value(settings.browser_id.as_deref().unwrap_or(""));
        
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let _ = on_settings_changed();
        }) as Box<dyn FnMut(_)>);
        
        elem.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // Initialize build info
    if let Some(span) = document.get_element_by_id("build-info")
        .and_then(|e| e.dyn_into::<web_sys::HtmlSpanElement>().ok()) {
        span.set_text_content(Some(BUILD_GIT_COMMIT_SHA));
    }
    
    Ok(())
}

fn on_settings_changed() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let default_bang = document.get_element_by_id("default-bang")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|e| e.value())
        .unwrap_or_else(|| Settings::default().default_bang);
    
    let bang_chars = document.get_element_by_id("bang-chars")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|e| e.value())
        .unwrap_or_else(|| Settings::default().bang_chars);
    
    let browser_id = document.get_element_by_id("browser-id")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
        .and_then(|e| {
            let val = e.value();
            if val.is_empty() { None } else { Some(val) }
        });
    
    let settings = Settings {
        default_bang,
        bang_chars,
        browser_id,
    };
    
    settings.write_to_cookie();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bangs_from_query() {
        let settings = Settings::default();
        
        let v1 = extract_bangs_from_query("hn hello world", &settings);
        assert_eq!(v1, Vec::<String>::new());
        
        let v2 = extract_bangs_from_query("hn hello world !g", &settings);
        assert_eq!(v2, vec!["g"]);
        
        let v3 = extract_bangs_from_query("hn hello !w world !g", &settings);
        assert_eq!(v3, vec!["g", "w"]);
        
        let v4 = extract_bangs_from_query("best json library !scala", &settings);
        assert_eq!(v4, vec!["scala"]);
        
        let v5 = extract_bangs_from_query("blah blah !scalaapi", &settings);
        assert_eq!(v5, vec!["scalaapi"]);
    }

    #[test]
    fn test_remove_bang_from_query() {
        let settings = Settings::default();
        
        let v1 = remove_bang_from_query("hn hello world !g", "g", None, &settings);
        assert_eq!(v1, "hn hello world");
        
        let v2 = remove_bang_from_query("hn hello !w world !g", "w", None, &settings);
        assert_eq!(v2, "hn hello world !g");
        
        let v3 = remove_bang_from_query("hn hello !w world !g", "g", None, &settings);
        assert_eq!(v3, "hn hello !w world");
        
        let v4 = remove_bang_from_query("!hn hello !w world !g", "hn", None, &settings);
        assert_eq!(v4, "hello !w world !g");
        
        let v5 = remove_bang_from_query("best json library !scala", "scala", Some("scala"), &settings);
        assert_eq!(v5, "best json library scala");
        
        let v6 = remove_bang_from_query("!scala best json library", "scala", Some("scala"), &settings);
        assert_eq!(v6, "scala best json library");
        
        let v7 = remove_bang_from_query("best !scala json library", "scala", Some("scala"), &settings);
        assert_eq!(v7, "best scala json library");
    }

    #[test]
    fn test_find_simple_engine_bangs() {
        let goog = find_bang_url_by_key("g");
        assert!(goog.is_some());
        let goog = goog.unwrap();
        assert!(goog.keys.contains(&"g".to_string()));
        assert!(goog.url.contains("google.com"));
        
        let brave = find_bang_url_by_key("br");
        assert!(brave.is_some());
        let brave = brave.unwrap();
        assert!(brave.keys.contains(&"br".to_string()));
        assert!(brave.url.contains("search.brave.com"));
        
        let github = find_bang_url_by_key("gh");
        assert!(github.is_some());
        let github = github.unwrap();
        assert!(github.keys.contains(&"gh".to_string()));
        assert!(github.url.contains("github.com"));
        
        let wikipedia_by_google = find_bang_url_by_key("gw");
        assert!(wikipedia_by_google.is_some());
        let wikipedia_by_google = wikipedia_by_google.unwrap();
        assert!(wikipedia_by_google.keys.contains(&"gw".to_string()));
        assert!(wikipedia_by_google.url.contains("google.com"));
        assert!(wikipedia_by_google.url.contains("wikipedia.org"));
    }
}
