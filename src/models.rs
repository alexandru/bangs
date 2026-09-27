//! The model definitions: [`Settings`], [`Bang`], [`QueryDef`], and
//! [`Referral`].

/// User preferences, persisted in the `settings` cookie.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// Bang key used when the query contains none (for example "b" for
    /// Brave).
    pub default_bang: String,
    /// Characters that mark a query token as a bang (default `!@/`).
    pub bang_chars: String,
    /// Whether safe-search engine variants are preferred.
    pub safe: bool,
    /// Optional browser identifier used to append referral tags.
    pub browser_id: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_bang: "b".to_string(),
            bang_chars: "!@/".to_string(),
            safe: true,
            browser_id: None,
        }
    }
}

impl Settings {
    /// Builds settings from raw cookie field values; missing, null, or
    /// empty fields fall back to their defaults, like the original dynamic
    /// `JSON.parse` accessors did.
    #[must_use]
    pub fn from_parts(
        default_bang: Option<String>,
        bang_chars: Option<String>,
        browser_id: Option<String>,
        safe: Option<bool>,
    ) -> Settings {
        let defaults = Settings::default();
        // Empty strings count as unset, like the Kotlin `nonEmptyOrNull`.
        Settings {
            default_bang: default_bang
                .filter(|value| !value.is_empty())
                .unwrap_or(defaults.default_bang),
            bang_chars: bang_chars
                .filter(|value| !value.is_empty())
                .unwrap_or(defaults.bang_chars),
            browser_id: browser_id.filter(|value| !value.is_empty()),
            safe: safe.unwrap_or(defaults.safe),
        }
    }

    /// Returns a copy with URL overrides applied; `None` for a parameter
    /// keeps the current value. Mirrors the Kotlin `overrideSettingsFromUrl`.
    #[must_use]
    pub fn with_url_overrides(
        &self,
        browser_id: Option<String>,
        default_bang: Option<String>,
        bang_chars: Option<String>,
        safe: Option<bool>,
    ) -> Settings {
        Settings {
            browser_id: browser_id.or_else(|| self.browser_id.clone()),
            default_bang: default_bang.unwrap_or_else(|| self.default_bang.clone()),
            bang_chars: bang_chars.unwrap_or_else(|| self.bang_chars.clone()),
            safe: safe.unwrap_or(self.safe),
        }
    }
}

/// Maps the `safe` URL parameter: "on" enables safe search, "off" disables
/// it, anything else keeps the current setting.
#[must_use]
pub fn parse_safe_flag(raw: &str) -> Option<bool> {
    match raw {
        "on" => Some(true),
        "off" => Some(false),
        _ => None,
    }
}

/// A resolved bang: the URL template (containing a `{{{s}}}` placeholder),
/// the keys it matched, and — for goggle-based bangs — the search context
/// word that replaces the bang token in the query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bang {
    pub url: String,
    pub keys: Vec<String>,
    pub search_context: Option<String>,
}

/// A static registry entry, converted into an owned [`Bang`] on lookup.
pub struct BangDef {
    pub url: &'static str,
    pub keys: &'static [&'static str],
    pub search_context: Option<&'static str>,
}

impl BangDef {
    /// Converts the static definition into an owned [`Bang`].
    pub fn to_bang(&self) -> Bang {
        Bang {
            url: self.url.to_string(),
            keys: self.keys.iter().map(|key| key.to_string()).collect(),
            search_context: self.search_context.map(|context| context.to_string()),
        }
    }
}

/// A site-restricted query template, combinable with general-purpose engine
/// keys: `wp` becomes `gwp` for Google, `dwp` for DuckDuckGo, and so on.
pub struct QueryDef {
    pub query: &'static str,
    pub keys: &'static [&'static str],
}

/// A referral tag appended to search URLs for a known browser.
#[derive(Debug, PartialEq)]
pub struct Referral {
    pub hostname: &'static str,
    pub browser_id: &'static str,
    pub referral: &'static str,
}

#[cfg(test)]
#[path = "../unit-tests/models.rs"]
mod tests;
