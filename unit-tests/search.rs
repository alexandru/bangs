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
        Some("https://www.google.com/search?q=hello&client=firefox-b-d&safe=active".to_string())
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
