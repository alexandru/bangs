use wasm_bindgen::prelude::*;
use crate::models::{Bang, Referral};
use crate::settings::{Settings, get_queries, get_general_purpose_engines, get_special_purpose_engines, get_referrals};
use js_sys::Date;

pub fn encode_uri_component(s: &str) -> String {
    js_sys::encode_uri_component(s).as_string().unwrap_or_default()
}

pub fn decode_uri_component(s: &str) -> String {
    js_sys::decode_uri_component(s)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| s.to_string())
}

pub fn read_cookie(name: &str) -> Option<String> {
    let document = web_sys::window()?.document()?;
    let cookie_string = js_sys::Reflect::get(&document, &JsValue::from_str("cookie"))
        .ok()?
        .as_string()?;
    
    // Simple pattern matching: "name=value"
    let pattern = format!("{}=", name);
    for part in cookie_string.split(';') {
        let trimmed = part.trim();
        if trimmed.starts_with(&pattern) {
            let value = &trimmed[pattern.len()..];
            return Some(decode_uri_component(value));
        }
    }
    None
}

pub fn write_cookie(name: &str, value: &str, days_until_expire: i32) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let encoded_value = encode_uri_component(value);
            let expires = Date::new(&JsValue::from(
                Date::now() + (days_until_expire as f64 * 24.0 * 60.0 * 60.0 * 1000.0)
            ));
            let expires_string = format!("expires={}", expires.to_utc_string());
            let hostname = window.location().hostname().unwrap_or_default();
            let cookie = format!(
                "{}={}; {}; path=/; domain={}",
                name, encoded_value, expires_string, hostname
            );
            let _ = js_sys::Reflect::set(&document, &JsValue::from_str("cookie"), &JsValue::from_str(&cookie));
        }
    }
}

pub fn read_settings_from_cookie() -> Option<Settings> {
    let json_str = read_cookie("settings")?;
    web_sys::console::log_1(&format!("Restoring settings: {}", json_str).into());
    Settings::from_json(&json_str)
}

pub fn override_settings_from_url(settings: &Settings) -> Settings {
    let browser_id = get_query_parameter("browserId").or_else(|| settings.browser_id.clone());
    let default_bang = get_query_parameter("defaultBang").unwrap_or_else(|| settings.default_bang.clone());
    let bang_chars = get_query_parameter("bangChars").unwrap_or_else(|| settings.bang_chars.clone());
    
    Settings {
        browser_id,
        default_bang,
        bang_chars,
    }
}

impl Settings {
    pub fn write_to_cookie(&self) {
        let json = self.to_json();
        web_sys::console::log_1(&format!("Saving settings: {}", json).into());
        write_cookie("settings", &json, 365 * 10);
    }
}

pub fn get_query_parameter(name: &str) -> Option<String> {
    let window = web_sys::window()?;
    let location = window.location();
    
    let search = location.search().ok()?;
    let hash = location.hash().ok()?;
    
    from_string(&search, name).or_else(|| from_string(&hash, name))
}

fn from_string(url: &str, name: &str) -> Option<String> {
    // Simple pattern matching: "name=value"
    let pattern = format!("{}=", name);
    
    for part in url.split(&['&', '?', '#']) {
        if part.starts_with(&pattern) {
            let value = &part[pattern.len()..];
            // Find the end of the value (before next & or end of string)
            let end = value.find('&').unwrap_or(value.len());
            let decoded = decode_uri_component(&value[..end]);
            let result = decoded.replace("+", " ");
            let trimmed = result.trim();
            return if trimmed.is_empty() { None } else { Some(trimmed.to_string()) };
        }
    }
    None
}

pub fn extract_bangs_from_query(raw_query: &str, settings: &Settings) -> Vec<String> {
    let parts: Vec<&str> = raw_query.split_whitespace().collect();
    let mut bangs = Vec::new();
    
    for part in parts {
        if !part.is_empty() && part.len() > 1 {
            let first_char = &part[0..1];
            if settings.bang_chars.contains(first_char) {
                bangs.push(part[1..].to_string());
            }
        }
    }
    
    bangs.reverse();
    bangs
}

pub fn find_bang_url_by_key(key: &str) -> Option<Bang> {
    // Special-purpose searches take precedence
    for bang in get_special_purpose_engines() {
        for bang_key in &bang.keys {
            if bang_key == key {
                return Some(bang.clone());
            }
        }
    }
    
    // Search through general-purpose engines
    for engine in get_general_purpose_engines() {
        for engine_key in &engine.keys {
            if engine_key == key {
                return Some(engine.clone());
            } else if key.starts_with(engine_key) {
                // Do we have a sub-query?
                let subkey = &key[engine_key.len()..];
                for query in get_queries() {
                    for query_key in &query.keys {
                        if query_key == subkey {
                            return Some(Bang {
                                url: engine.url.replace("{{{s}}}", &query.query),
                                keys: vec![format!("{}{}", engine_key, query_key)],
                                search_context: None,
                            });
                        }
                    }
                }
            }
        }
    }
    
    None
}

pub fn remove_bang_from_query(
    query: &str,
    bang: &str,
    replacement: Option<&str>,
    settings: &Settings,
) -> String {
    // Build search patterns
    let search_patterns: Vec<String> = settings.bang_chars.chars()
        .map(|c| format!("{}{}", c, bang))
        .collect();
    
    // Find the last occurrence of the bang pattern
    let mut last_pos = None;
    let mut matched_len = 0;
    
    for pattern in &search_patterns {
        if let Some(pos) = query.rfind(pattern) {
            // Check if it's at a word boundary
            let is_valid_start = pos == 0 || query.chars().nth(pos - 1).map_or(false, |c| c.is_whitespace());
            let is_valid_end = pos + pattern.len() >= query.len() || 
                query.chars().nth(pos + pattern.len()).map_or(false, |c| c.is_whitespace());
            
            if is_valid_start && is_valid_end && (last_pos.is_none() || pos > last_pos.unwrap()) {
                last_pos = Some(pos);
                matched_len = pattern.len();
            }
        }
    }
    
    if let Some(pos) = last_pos {
        let mut result = String::new();
        
        // Add text before the bang
        let before = &query[..pos];
        result.push_str(before.trim_end());
        
        // Add replacement if provided
        if let Some(repl) = replacement {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(repl);
        }
        
        // Add text after the bang
        let after = &query[pos + matched_len..];
        let after_trimmed = after.trim_start();
        if !after_trimmed.is_empty() {
            if !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
            result.push_str(after_trimmed);
        }
        
        result
    } else {
        query.to_string()
    }
}

pub fn redirect_to_url(url: &str, debug: bool) -> Result<(), JsValue> {
    if debug {
        return Err(JsValue::from_str(&format!("Redirect to {}", url)));
    }
    
    let window = web_sys::window().expect("no global `window` exists");
    window.location().replace(url)?;
    Ok(())
}

pub fn find_referral(settings: &Settings, url: &str) -> Option<Referral> {
    let browser_id = settings.browser_id.as_ref()?;
    
    if !url.contains("?") {
        return None;
    }
    
    for referral in get_referrals() {
        if url.contains(&referral.hostname) 
            && browser_id.trim().eq_ignore_ascii_case(&referral.browser_id) {
            return Some(referral);
        }
    }
    
    None
}
