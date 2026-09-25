//! Reading and writing `document.cookie` strings.

/// Extracts the raw, still percent-encoded value of `name` from a
/// `document.cookie` string (for example `"a=1; settings=%7B%7D"`).
///
/// This mirrors the Kotlin regex `name=([^;]+)`: the first occurrence wins,
/// `name` may match as a substring of another cookie's name, and an empty
/// value counts as missing.
#[must_use]
pub fn find_cookie_value<'a>(cookie_header: &'a str, name: &str) -> Option<&'a str> {
    if name.is_empty() {
        return None;
    }
    let bytes = cookie_header.as_bytes();
    let mut search_from = 0;
    loop {
        // Occurrences of `name` that overlap a previously rejected one
        // cannot happen for the fixed names used in this app.
        let offset = cookie_header[search_from..].find(name)? + search_from;
        let value_start = offset + name.len();
        if bytes.get(value_start) == Some(&b'=') {
            // `value_start` points at an ASCII byte, so slicing is safe.
            let value = &cookie_header[value_start + 1..];
            let end = value.find(';').unwrap_or(value.len());
            let raw = &value[..end];
            if raw.is_empty() {
                return None;
            }
            return Some(raw);
        }
        search_from = value_start;
        if search_from >= bytes.len() {
            return None;
        }
    }
}

/// Builds the value assigned to `document.cookie` when writing a cookie,
/// with the exact layout the original app used. `expires_utc` must be
/// formatted like JavaScript's `Date#toUTCString()`
/// (for example `"Wed, 21 Oct 2026 07:28:00 GMT"`).
#[must_use]
pub fn build_set_cookie(
    name: &str,
    encoded_value: &str,
    expires_utc: &str,
    hostname: &str,
) -> String {
    format!("{name}={encoded_value}; expires={expires_utc}; path=/; domain={hostname}")
}

#[cfg(test)]
mod tests {
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
}
