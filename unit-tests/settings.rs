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
fn from_parts_falls_back_to_defaults_for_missing_fields() {
    let settings = Settings::from_parts(None, None, None, None);
    assert_eq!(settings, Settings::default());
}

#[test]
fn from_parts_falls_back_to_defaults_for_empty_fields() {
    let settings = Settings::from_parts(
        Some(String::new()),
        Some(String::new()),
        Some(String::new()),
        None,
    );
    assert_eq!(settings, Settings::default());
}

#[test]
fn from_parts_reads_all_fields() {
    let settings = Settings::from_parts(
        Some("g".to_string()),
        Some("/".to_string()),
        Some("vivaldi".to_string()),
        Some(false),
    );
    assert_eq!(settings.default_bang, "g");
    assert_eq!(settings.bang_chars, "/");
    assert_eq!(settings.browser_id, Some("vivaldi".to_string()));
    assert!(!settings.safe);
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
