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
#[path = "../unit-tests/url_codec.rs"]
mod tests;
