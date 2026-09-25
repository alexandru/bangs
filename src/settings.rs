//! User settings, persisted as JSON in the `settings` cookie.

use serde::{Deserialize, Serialize};

/// User preferences, persisted in the `settings` cookie as JSON.
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
    /// Parses the settings cookie JSON. Returns `None` when the JSON is
    /// malformed; missing, null, or empty fields fall back to their
    /// defaults, like the original dynamic `JSON.parse` accessors did.
    #[must_use]
    pub fn from_json(json: &str) -> Option<Settings> {
        let dto: SettingsDto = serde_json::from_str(json).ok()?;
        Some(Settings::from(dto))
    }

    /// Serializes the settings for the cookie, with the same field order
    /// the original `JSON.stringify` produced.
    #[must_use]
    pub fn to_json(&self) -> String {
        let dto = SettingsDto::from(self);
        // `SettingsDto` is a plain struct, so serialization cannot fail;
        // were it to fail, "{}" would simply parse back to the defaults.
        serde_json::to_string(&dto).unwrap_or_else(|_| "{}".to_string())
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

/// Wire format of the settings cookie. All fields are optional so that
/// missing or null values degrade to defaults instead of failing the whole
/// cookie.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsDto {
    default_bang: Option<String>,
    bang_chars: Option<String>,
    browser_id: Option<String>,
    safe: Option<bool>,
}

impl From<SettingsDto> for Settings {
    fn from(dto: SettingsDto) -> Settings {
        let defaults = Settings::default();
        Settings {
            // Empty strings count as unset, like the Kotlin `nonEmptyOrNull`.
            default_bang: dto
                .default_bang
                .filter(|value| !value.is_empty())
                .unwrap_or(defaults.default_bang),
            bang_chars: dto
                .bang_chars
                .filter(|value| !value.is_empty())
                .unwrap_or(defaults.bang_chars),
            browser_id: dto.browser_id.filter(|value| !value.is_empty()),
            safe: dto.safe.unwrap_or(defaults.safe),
        }
    }
}

impl From<&Settings> for SettingsDto {
    fn from(settings: &Settings) -> SettingsDto {
        SettingsDto {
            default_bang: Some(settings.default_bang.clone()),
            bang_chars: Some(settings.bang_chars.clone()),
            // `None` serializes as JSON `null`, like `JSON.stringify` did.
            browser_id: settings.browser_id.clone(),
            safe: Some(settings.safe),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_original_app() {
        let settings = Settings::default();
        assert_eq!(settings.default_bang, "b");
        assert_eq!(settings.bang_chars, "!@/");
        assert!(settings.safe);
        assert_eq!(settings.browser_id, None);
    }

    #[test]
    fn to_json_uses_the_original_field_order_and_camel_case() {
        let json = Settings::default().to_json();
        assert_eq!(
            json,
            r#"{"defaultBang":"b","bangChars":"!@/","browserId":null,"safe":true}"#
        );
    }

    #[test]
    fn from_json_round_trips() {
        let settings = Settings {
            default_bang: "g".to_string(),
            bang_chars: "/".to_string(),
            safe: false,
            browser_id: Some("firefox".to_string()),
        };
        assert_eq!(Settings::from_json(&settings.to_json()), Some(settings));
    }

    #[test]
    fn from_json_falls_back_to_defaults_for_missing_or_empty_fields() {
        let settings = Settings::from_json("{}").expect("valid json");
        assert_eq!(settings, Settings::default());

        let settings =
            Settings::from_json(r#"{"defaultBang":"","browserId":""}"#).expect("valid json");
        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn from_json_reads_all_fields() {
        let json = r#"{"defaultBang":"g","bangChars":"/","browserId":"vivaldi","safe":false}"#;
        let settings = Settings::from_json(json).expect("valid json");
        assert_eq!(settings.default_bang, "g");
        assert_eq!(settings.bang_chars, "/");
        assert_eq!(settings.browser_id, Some("vivaldi".to_string()));
        assert!(!settings.safe);
    }

    #[test]
    fn from_json_ignores_extra_keys_but_rejects_malformed_input() {
        let settings =
            Settings::from_json(r#"{"defaultBang":"g","newField":1}"#).expect("valid json");
        assert_eq!(settings.default_bang, "g");
        assert_eq!(Settings::from_json("{not json"), None);
        // Wrongly-typed values reject the whole cookie instead of crashing
        // (the Kotlin version would have thrown a cast exception).
        assert_eq!(Settings::from_json(r#"{"safe":"yes"}"#), None);
    }

    #[test]
    fn parse_safe_flag_maps_on_and_off() {
        assert_eq!(parse_safe_flag("on"), Some(true));
        assert_eq!(parse_safe_flag("off"), Some(false));
        assert_eq!(parse_safe_flag("anything"), None);
    }

    #[test]
    fn with_url_overrides_replaces_only_given_values() {
        let base = Settings::default();
        let overridden = base.with_url_overrides(
            Some("firefox".to_string()),
            Some("g".to_string()),
            Some("/".to_string()),
            Some(false),
        );
        assert_eq!(overridden.browser_id, Some("firefox".to_string()));
        assert_eq!(overridden.default_bang, "g");
        assert_eq!(overridden.bang_chars, "/");
        assert!(!overridden.safe);

        let unchanged = base.with_url_overrides(None, None, None, None);
        assert_eq!(unchanged, base);
    }
}
