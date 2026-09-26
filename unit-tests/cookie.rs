use super::*;

#[test]
fn find_cookie_value_returns_the_first_value() {
    assert_eq!(
        find_cookie_value("settings=%7B%7D; other=x", "settings"),
        Some("%7B%7D")
    );
    assert_eq!(
        find_cookie_value("a=1; settings=abc", "settings"),
        Some("abc")
    );
    assert_eq!(
        find_cookie_value("settings=a; settings=b", "settings"),
        Some("a")
    );
}

#[test]
fn find_cookie_value_ignores_empty_and_missing_values() {
    assert_eq!(find_cookie_value("settings=", "settings"), None);
    assert_eq!(find_cookie_value("settings=; a=1", "settings"), None);
    assert_eq!(find_cookie_value("", "settings"), None);
    assert_eq!(find_cookie_value("a=1", "settings"), None);
}

#[test]
fn find_cookie_value_matches_names_as_substrings() {
    // The original regex had no word boundary; keep the same behavior.
    assert_eq!(find_cookie_value("prefsettings=1", "settings"), Some("1"));
    assert_eq!(find_cookie_value("xsettings2=1", "settings"), None);
}

#[test]
fn build_set_cookie_formats_the_original_layout() {
    let cookie = build_set_cookie(
        "settings",
        "%7B%7D",
        "Wed, 21 Oct 2026 07:28:00 GMT",
        "bangs.alexn.org",
    );
    assert_eq!(
        cookie,
        "settings=%7B%7D; expires=Wed, 21 Oct 2026 07:28:00 GMT; path=/; domain=bangs.alexn.org"
    );
}
