//! Pure search logic: extracting bangs from queries, removing them, and
//! resolving the redirect target.

use crate::bangs::{self, Bang, DEFAULT_BANG_KEY, Referral, find_bang};
use crate::settings::Settings;
use crate::url_codec;

/// Collects the bang keys from `raw_query`, in reverse order so that the
/// last bang in the query is tried first. A token is a bang when its first
/// character is one of the configured bang chars; the rest of the token is
/// the key.
#[must_use]
pub fn extract_bangs(raw_query: &str, settings: &Settings) -> Vec<String> {
    let mut bangs: Vec<String> = Vec::new();
    for token in raw_query.split(is_js_whitespace) {
        let Some(first) = token.chars().next() else {
            continue;
        };
        if !settings.bang_chars.contains(first) {
            continue;
        }
        // Byte-safe strip of the first character, whatever its width.
        let rest = match token.char_indices().nth(1) {
            Some((offset, _)) => &token[offset..],
            None => "",
        };
        bangs.push(rest.to_string());
    }
    bangs.reverse();
    bangs
}

/// Removes the last occurrence of `bang` from `query` and trims the result.
///
/// With a `replacement` (a bang's search context) the bang token is swapped
/// for it in place: `!scala best json library` becomes
/// `scala best json library`. Without one, the token plus the whitespace
/// around it collapses into a single space. This reproduces the two Kotlin
/// regexes, including their consuming and lookaround whitespace behavior:
///
/// - removal: `(^|\s+|\b)[<bang_chars>]<bang>($|\s+|\b)` → `" "`
/// - replace: `(?<=^|\b|\s+)[<bang_chars>]<bang>(?=$|\b|\s+)` → context
///
/// Only the last match is replaced.
#[must_use]
pub fn remove_bang(
    query: &str,
    bang: &str,
    replacement: Option<&str>,
    settings: &Settings,
) -> String {
    let range = find_last_bang_range(query, bang, &settings.bang_chars, replacement.is_some());
    let Some((start, end)) = range else {
        return query.trim().to_string();
    };
    let replacement = replacement.unwrap_or(" ");
    let mut result = String::with_capacity(query.len() + replacement.len());
    result.push_str(&query[..start]);
    result.push_str(replacement);
    result.push_str(&query[end..]);
    result.trim().to_string()
}

/// Finds the referral tag for `url` when `settings.browser_id` names the
/// browser it belongs to (case-insensitively). `None` without a browser id
/// or when the URL has no query string to append to.
#[must_use]
pub fn find_referral(settings: &Settings, url: &str) -> Option<&'static Referral> {
    let browser_id = settings.browser_id.as_deref()?;
    if !url.contains('?') {
        return None;
    }
    bangs::REFERRALS.iter().find(|referral| {
        url.contains(referral.hostname)
            && browser_id.trim().eq_ignore_ascii_case(referral.browser_id)
    })
}

/// Resolves the redirect target for `raw_query`: bang extraction, engine
/// lookup, the default-bang fallback, and URL construction. The encoded
/// query (with an optional referral tag) is substituted for the `{{{s}}}`
/// placeholder. Returns `None` only if even the fallback
/// [`DEFAULT_BANG_KEY`] is missing from the registry.
#[must_use]
pub fn resolve_target_url(raw_query: &str, settings: &Settings) -> Option<String> {
    let mut found_bang: Option<Bang> = None;
    let mut query = raw_query.to_string();
    for bang in extract_bangs(raw_query, settings) {
        let Some(b) = find_bang(&bang, settings.safe) else {
            continue;
        };
        query = remove_bang(raw_query, &bang, b.search_context.as_deref(), settings);
        found_bang = Some(b);
        break;
    }
    let found_bang = found_bang
        .or_else(|| find_bang(&settings.default_bang, settings.safe))
        .or_else(|| find_bang(DEFAULT_BANG_KEY, settings.safe))?;

    let mut search_value = url_codec::encode_uri_component(&query);
    if let Some(referral) = find_referral(settings, &found_bang.url) {
        search_value.push('&');
        search_value.push_str(referral.referral);
    }
    Some(found_bang.url.replace("{{{s}}}", &search_value))
}

/// JavaScript regex `\s`: the character class JS engines treat as whitespace
/// (wider than Rust's `char::is_whitespace`, and it includes `\u{feff}`).
fn is_js_whitespace(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\x0b' | '\x0c' | '\r' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// JavaScript regex `\w`: word characters (ASCII only, unlike Rust's
/// `char::is_alphanumeric`).
fn is_js_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Finds the byte range of the last bang match, replicating the global
/// left-to-right, non-overlapping scan of the Kotlin regexes. Returns
/// `(start, end)` byte offsets into `query`.
///
/// `use_lookarounds` switches between the two Kotlin patterns: lookarounds
/// match only the bang token, while the consuming variant also swallows the
/// whitespace run before and/or after it (which the single-space
/// replacement then collapses).
fn find_last_bang_range(
    query: &str,
    bang: &str,
    bang_chars: &str,
    use_lookarounds: bool,
) -> Option<(usize, usize)> {
    let chars: Vec<(usize, char)> = query.char_indices().collect();
    let bang_text: Vec<char> = bang.chars().collect();
    let mut last: Option<(usize, usize)> = None;
    // Byte offset where the next match may start: the end of the last
    // match, so that matches never overlap, like with a global regex scan.
    let mut resume = 0;

    for (idx, &(bang_char_offset, c)) in chars.iter().enumerate() {
        if !bang_chars.contains(c) {
            continue;
        }
        // The bang text must follow the bang char.
        let after_text_idx = idx + 1 + bang_text.len();
        if after_text_idx > chars.len() {
            continue;
        }
        let text_matches = (idx + 1..after_text_idx)
            .zip(bang_text.iter())
            .all(|(i, expected)| chars[i].1 == *expected);
        if !text_matches {
            continue;
        }
        let after_text_offset = if after_text_idx < chars.len() {
            chars[after_text_idx].0
        } else {
            query.len()
        };

        let (match_start, match_end) = if use_lookarounds {
            // (?<=^|\b|\s+): start of string, a word boundary, or
            // whitespace before the bang char.
            let boundary_before = idx == 0 || {
                let prev = chars[idx - 1].1;
                is_js_whitespace(prev) || is_js_word(prev) != is_js_word(c)
            };
            if !boundary_before {
                continue;
            }
            // (?=$|\b|\s+): end of string, a word boundary, or whitespace
            // after the bang text.
            let boundary_after = after_text_idx == chars.len() || {
                let next = chars[after_text_idx].1;
                let last_text_char = chars[after_text_idx - 1].1;
                is_js_whitespace(next) || is_js_word(last_text_char) != is_js_word(next)
            };
            if !boundary_after {
                continue;
            }
            (bang_char_offset, after_text_offset)
        } else {
            // ($|\s+|\b): consume a trailing whitespace run, or stop at a
            // word boundary, or require end of string.
            let match_end = if after_text_idx == chars.len() {
                after_text_offset
            } else {
                let next = chars[after_text_idx].1;
                let last_text_char = chars[after_text_idx - 1].1;
                if is_js_whitespace(next) {
                    // Greedy `\s+`: swallow the whole run.
                    let mut end_idx = after_text_idx;
                    while end_idx < chars.len() && is_js_whitespace(chars[end_idx].1) {
                        end_idx += 1;
                    }
                    if end_idx < chars.len() {
                        chars[end_idx].0
                    } else {
                        query.len()
                    }
                } else if is_js_word(last_text_char) != is_js_word(next) {
                    after_text_offset
                } else {
                    continue;
                }
            };
            // (^|\s+|\b): consume the whitespace run before the bang char,
            // or match an empty word boundary. The regex scans from the
            // left, so the run start (further left) wins over the boundary.
            let match_start = if idx == 0 {
                0
            } else {
                let prev = chars[idx - 1].1;
                if is_js_whitespace(prev) {
                    let mut start_idx = idx - 1;
                    while start_idx > 0 && is_js_whitespace(chars[start_idx - 1].1) {
                        start_idx -= 1;
                    }
                    chars[start_idx].0
                } else if is_js_word(prev) != is_js_word(c) {
                    bang_char_offset
                } else {
                    continue;
                }
            };
            (match_start, match_end)
        };

        // Matches may not overlap the previous one.
        if match_start < resume {
            continue;
        }
        last = Some((match_start, match_end));
        resume = match_end;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_bangs_finds_nothing_in_a_plain_query() {
        let bangs = extract_bangs("hn hello world", &Settings::default());
        assert_eq!(bangs, Vec::<String>::new());
    }

    #[test]
    fn extract_bangs_finds_a_trailing_bang() {
        let bangs = extract_bangs("hn hello world !g", &Settings::default());
        assert_eq!(bangs, vec!["g".to_string()]);
    }

    #[test]
    fn extract_bangs_reversed_last_bang_first() {
        let bangs = extract_bangs("hn hello !w world !g", &Settings::default());
        assert_eq!(bangs, vec!["g".to_string(), "w".to_string()]);
    }

    #[test]
    fn extract_bangs_recognizes_long_keys() {
        let bangs = extract_bangs("blah blah !scalaapi", &Settings::default());
        assert_eq!(bangs, vec!["scalaapi".to_string()]);
    }

    #[test]
    fn extract_bangs_honors_custom_bang_chars() {
        let settings = Settings {
            bang_chars: "/".to_string(),
            ..Settings::default()
        };
        assert_eq!(
            extract_bangs("/so hello", &settings),
            vec!["so".to_string()]
        );
    }

    #[test]
    fn remove_bang_drops_a_trailing_bang() {
        let settings = Settings::default();
        assert_eq!(
            remove_bang("hn hello world !g", "g", None, &settings),
            "hn hello world"
        );
    }

    #[test]
    fn remove_bang_drops_a_middle_or_trailing_bang() {
        let settings = Settings::default();
        assert_eq!(
            remove_bang("hn hello !w world !g", "w", None, &settings),
            "hn hello world !g"
        );
        assert_eq!(
            remove_bang("hn hello !w world !g", "g", None, &settings),
            "hn hello !w world"
        );
    }

    #[test]
    fn remove_bang_drops_a_leading_bang() {
        let settings = Settings::default();
        assert_eq!(
            remove_bang("!hn hello !w world !g", "hn", None, &settings),
            "hello !w world !g"
        );
    }

    #[test]
    fn remove_bang_replaces_the_search_context_in_place() {
        let settings = Settings::default();
        assert_eq!(
            remove_bang(
                "best json library !scala",
                "scala",
                Some("scala"),
                &settings
            ),
            "best json library scala"
        );
        assert_eq!(
            remove_bang(
                "!scala best json library",
                "scala",
                Some("scala"),
                &settings
            ),
            "scala best json library"
        );
        assert_eq!(
            remove_bang(
                "best !scala json library",
                "scala",
                Some("scala"),
                &settings
            ),
            "best scala json library"
        );
    }

    #[test]
    fn remove_bang_replaces_only_the_last_occurrence() {
        let settings = Settings::default();
        assert_eq!(remove_bang("!g hello !g", "g", None, &settings), "!g hello");
        assert_eq!(
            remove_bang("!scala x !scala", "scala", Some("scala"), &settings),
            "!scala x scala"
        );
    }

    #[test]
    fn remove_bang_matches_bangs_without_a_space_before() {
        // "hello!w" is still a bang: \b matches between "o" and "!".
        let settings = Settings::default();
        assert_eq!(
            remove_bang("hello!w world", "w", None, &settings),
            "hello world"
        );
    }

    #[test]
    fn remove_bang_ignores_occurrences_without_a_boundary() {
        let settings = Settings::default();
        // "-" is neither whitespace nor a word char, so no boundary exists.
        assert_eq!(
            remove_bang("hello-!w world", "w", None, &settings),
            "hello-!w world"
        );
        // The bang text must end at a boundary too: "world" swallows "w".
        assert_eq!(
            remove_bang("hello !world x", "w", None, &settings),
            "hello !world x"
        );
    }

    #[test]
    fn remove_bang_collapses_surrounding_whitespace() {
        let settings = Settings::default();
        assert_eq!(remove_bang("  !g  hello", "g", None, &settings), "hello");
        assert_eq!(
            remove_bang("hello !w  world", "w", None, &settings),
            "hello world"
        );
    }

    #[test]
    fn remove_bang_returns_trimmed_query_without_a_match() {
        let settings = Settings::default();
        assert_eq!(
            remove_bang("  hello world  ", "g", None, &settings),
            "hello world"
        );
    }

    #[test]
    fn find_referral_matches_browser_id_case_insensitively() {
        let settings = Settings {
            browser_id: Some("FireFox".to_string()),
            ..Settings::default()
        };
        let referral = find_referral(&settings, "https://www.google.com/search?q=x");
        assert_eq!(referral.map(|r| r.referral), Some("client=firefox-b-d"));
        let referral = find_referral(&settings, "https://duckduckgo.com/?q=x");
        assert_eq!(referral.map(|r| r.referral), Some("t=ffab"));
    }

    #[test]
    fn find_referral_requires_browser_id_and_query_string() {
        let settings = Settings::default();
        assert_eq!(
            find_referral(&settings, "https://www.google.com/search?q=x"),
            None
        );

        let settings = Settings {
            browser_id: Some("firefox".to_string()),
            ..Settings::default()
        };
        assert_eq!(
            find_referral(&settings, "https://www.google.com/search"),
            None
        );
        assert_eq!(find_referral(&settings, "https://example.com/?q=x"), None);

        let settings = Settings {
            browser_id: Some("vivaldi".to_string()),
            ..Settings::default()
        };
        let referral = find_referral(&settings, "https://www.startpage.com/do/search?query=x");
        assert_eq!(
            referral.map(|r| r.referral),
            Some("segment=startpage.vivaldi")
        );
    }

    #[test]
    fn resolve_target_url_uses_the_settings_default_engine() {
        let url = resolve_target_url("hello world", &Settings::default());
        assert_eq!(
            url,
            Some("https://safe.search.brave.com/search?q=hello%20world".to_string())
        );
    }

    #[test]
    fn resolve_target_url_falls_back_to_the_global_default_bang() {
        let settings = Settings {
            default_bang: "does-not-exist".to_string(),
            ..Settings::default()
        };
        let url = resolve_target_url("hello world", &settings);
        assert_eq!(
            url,
            Some("https://www.google.com/search?q=hello%20world&safe=active".to_string())
        );
    }

    #[test]
    fn resolve_target_url_removes_the_bang_and_encodes_the_query() {
        let url = resolve_target_url("!w hello world", &Settings::default());
        assert_eq!(
            url,
            Some("https://en.wikipedia.org/wiki/Special:Search?search=hello%20world".to_string())
        );
    }

    #[test]
    fn resolve_target_url_replaces_the_search_context() {
        let url = resolve_target_url("!scala best json library", &Settings::default());
        assert_eq!(
            url,
            Some(
                "https://search.brave.com/goggles?q=scala%20best%20json%20library&source=web&\
                 goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fscala"
                    .to_string()
            )
        );
    }

    #[test]
    fn resolve_target_url_appends_the_referral_tag() {
        let settings = Settings {
            browser_id: Some("firefox".to_string()),
            ..Settings::default()
        };
        let url = resolve_target_url("!g hello", &settings);
        assert_eq!(
            url,
            Some(
                "https://www.google.com/search?q=hello&client=firefox-b-d&safe=active".to_string()
            )
        );
    }

    #[test]
    fn resolve_target_url_uses_the_last_bang_in_the_query() {
        let url = resolve_target_url("!w hello !g world", &Settings::default());
        assert_eq!(
            url,
            Some("https://www.google.com/search?q=!w%20hello%20world&safe=active".to_string())
        );
    }
}
