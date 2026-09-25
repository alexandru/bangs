//! Percent-encoding and query-string scanning with the exact semantics of
//! the JavaScript functions the original app called (`encodeURIComponent`
//! and `decodeURIComponent`), plus the raw scan of `location.search` /
//! `location.hash`.

/// Characters `encodeURIComponent` leaves untouched, besides ASCII letters
/// and digits: `-_.!~*'()`.
const ENCODE_URI_COMPONENT_SAFE: [u8; 9] = *b"-_.!~*'()";

/// Encodes a string like JavaScript's `encodeURIComponent`: every character
/// other than ASCII letters, digits, and `-_.!~*'()` becomes percent-encoded
/// UTF-8 bytes with uppercase hex digits.
#[must_use]
pub fn encode_uri_component(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || ENCODE_URI_COMPONENT_SAFE.contains(&byte) {
            // All safe bytes are ASCII, so the cast cannot split a
            // multi-byte character.
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(hex_digit(byte >> 4));
            encoded.push(hex_digit(byte & 0x0f));
        }
    }
    encoded
}

/// Decodes percent-encoded UTF-8 like JavaScript's `decodeURIComponent`.
/// Returns `None` for malformed input (a stray `%`, invalid hex digits, or a
/// byte sequence that is not valid UTF-8), where the JS function would have
/// thrown an exception.
#[must_use]
pub fn decode_uri_component(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                let high = hex_value(*bytes.get(i + 1)?)?;
                let low = hex_value(*bytes.get(i + 2)?)?;
                decoded.push((high << 4) | low);
                i += 3;
            }
            byte => {
                decoded.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8(decoded).ok()
}

/// Scans `source` (a `location.search` or `location.hash` string) for the
/// first `[&?#]{name}=` occurrence and returns the raw, still percent-encoded
/// value, which ends at the next `&`. Mirrors the Kotlin regex
/// `[&?#]{name}=([^&]*)`, including that `#` may both start the scan and
/// appear inside a value.
#[must_use]
pub fn find_query_param<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let bytes = source.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if !matches!(byte, b'&' | b'?' | b'#') {
            continue;
        }
        // `byte` is ASCII, so `i + 1` is a char boundary.
        let rest = &source[i + 1..];
        if !rest.starts_with(name) {
            continue;
        }
        let value_start = i + 1 + name.len();
        if bytes.get(value_start) != Some(&b'=') {
            continue;
        }
        let value = &source[value_start + 1..];
        let end = value.find('&').unwrap_or(value.len());
        return Some(&value[..end]);
    }
    None
}

/// Full query-parameter post-processing: percent-decode, turn `+` into a
/// space, trim, and drop empty results — the Kotlin pipeline
/// `decodeURIComponent(it).replace("+", " ")?.trim().nonEmptyOrNull()`.
#[must_use]
pub fn decode_query_value(raw: &str) -> Option<String> {
    let decoded = decode_uri_component(raw)?;
    let value = decoded.replace('+', " ");
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

/// Maps a hex digit to its numeric value; `None` for anything else.
fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Maps a nibble to an uppercase hex digit, as produced by JS encoding.
fn hex_digit(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'A' + nibble - 10) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_keeps_uri_component_safe_characters() {
        let safe = "AZaz09-_.!~*'()";
        assert_eq!(encode_uri_component(safe), safe);
    }

    #[test]
    fn encode_percent_encodes_the_rest() {
        assert_eq!(encode_uri_component("hello world"), "hello%20world");
        assert_eq!(encode_uri_component("a+b/c"), "a%2Bb%2Fc");
        assert_eq!(encode_uri_component("100%"), "100%25");
        assert_eq!(encode_uri_component("é"), "%C3%A9");
        assert_eq!(encode_uri_component("…"), "%E2%80%A6");
    }

    #[test]
    fn decode_reverses_encoding() {
        assert_eq!(
            decode_uri_component("hello%20world"),
            Some("hello world".to_string())
        );
        assert_eq!(decode_uri_component("%C3%A9"), Some("é".to_string()));
        // '+' is a literal here; the space mapping happens in
        // `decode_query_value`.
        assert_eq!(decode_uri_component("a+b"), Some("a+b".to_string()));
        assert_eq!(decode_uri_component(""), Some(String::new()));
    }

    #[test]
    fn decode_rejects_malformed_input() {
        assert_eq!(decode_uri_component("100%"), None);
        assert_eq!(decode_uri_component("%zz"), None);
        assert_eq!(decode_uri_component("%C3"), None); // truncated UTF-8
        assert_eq!(decode_uri_component("%ED%A0%BD"), None); // lone surrogate
    }

    #[test]
    fn find_query_param_scans_search_and_hash_strings() {
        assert_eq!(find_query_param("?q=hello+world", "q"), Some("hello+world"));
        assert_eq!(find_query_param("?a=1&q=2", "q"), Some("2"));
        assert_eq!(find_query_param("#q=x%20y", "q"), Some("x%20y"));
        assert_eq!(find_query_param("?qq=1&q=2", "q"), Some("2"));
        assert_eq!(find_query_param("?q=a#b", "q"), Some("a#b"));
        assert_eq!(find_query_param("?q=", "q"), Some(""));
        assert_eq!(find_query_param("?debug", "q"), None);
        assert_eq!(find_query_param("", "q"), None);
        assert_eq!(find_query_param("?defaultBang=g", "defaultBang"), Some("g"));
    }

    #[test]
    fn decode_query_value_postprocesses_like_the_original() {
        assert_eq!(
            decode_query_value("hello+world"),
            Some("hello world".to_string())
        );
        assert_eq!(decode_query_value("%C3%A9"), Some("é".to_string()));
        assert_eq!(decode_query_value("  x  "), Some("x".to_string()));
        assert_eq!(decode_query_value(""), None);
        assert_eq!(decode_query_value("+"), None); // decodes to a space, trims to nothing
        assert_eq!(decode_query_value("%zz"), None);
    }
}
