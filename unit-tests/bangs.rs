use super::*;

#[test]
fn find_bang_resolves_simple_engines() {
    let goog = find_bang("g", false).expect("google");
    assert!(goog.keys.iter().any(|key| key == "g"));
    assert!(goog.url.contains("google.com"));

    let brave = find_bang("br", false).expect("brave");
    assert!(brave.keys.iter().any(|key| key == "br"));
    assert!(brave.url.contains("search.brave.com"));

    let github = find_bang("gh", false).expect("github");
    assert!(github.keys.iter().any(|key| key == "gh"));
    assert!(github.url.contains("github.com"));
}

#[test]
fn find_bang_synthesizes_engine_plus_site_query() {
    let wikipedia_by_google = find_bang("gw", false).expect("gw");
    assert!(wikipedia_by_google.keys.iter().any(|key| key == "gw"));
    assert_eq!(
        wikipedia_by_google.url,
        "https://www.google.com/search?q=site:wikipedia.org%20{{{s}}}"
    );

    let hn_by_google = find_bang("ghn", false).expect("ghn");
    assert_eq!(hn_by_google.keys, vec!["ghn".to_string()]);
    assert!(hn_by_google.url.contains("news.ycombinator.com"));
}

#[test]
fn find_bang_prefers_special_purpose_engines() {
    // "w" is Wikipedia directly, not Google + Wikipedia.
    let wikipedia = find_bang("w", false).expect("w");
    assert!(wikipedia.url.contains("en.wikipedia.org"));

    let scala_goggles = find_bang("scala", false).expect("scala");
    assert_eq!(scala_goggles.search_context, Some("scala".to_string()));
}

#[test]
fn find_bang_returns_none_for_unknown_keys() {
    assert!(find_bang("gx", false).is_none());
    assert!(find_bang("", false).is_none());
    assert!(find_bang("!", false).is_none());
}

#[test]
fn find_bang_resolves_marginalia_bang() {
    assert_eq!(
        find_bang("ma", false).map(|bang| bang.url),
        Some("https://marginalia-search.com/search?query={{{s}}}".to_string())
    );
    assert_eq!(
        find_bang("ma", true).map(|bang| bang.url),
        Some("https://marginalia-search.com/search?query={{{s}}}&nsfw=smut".to_string())
    );
}

#[test]
fn find_bang_resolves_mojeek_bang() {
    assert_eq!(
        find_bang("mo", false).map(|bang| bang.url),
        Some("https://www.mojeek.com/search?q={{{s}}}".to_string())
    );
    assert_eq!(
        find_bang("mo", true).map(|bang| bang.url),
        Some("https://www.mojeek.com/search?q={{{s}}}&safe=1".to_string())
    );
}

#[test]
fn find_bang_resolves_safe_engine_variants() {
    let goog = find_bang("g", true).expect("google");
    assert!(goog.url.contains("safe=active"));
    let brave = find_bang("br", true).expect("brave");
    assert!(brave.url.contains("safe.search.brave.com"));
    let duck = find_bang("d", true).expect("duckduckgo");
    assert!(duck.url.contains("safe.duckduckgo.com"));
    let startpage = find_bang("s", true).expect("startpage");
    assert!(startpage.url.contains("safe.startpage.com"));
    let qwant = find_bang("q", true).expect("qwant");
    assert!(qwant.url.contains("safesearch=2"));
    let bing = find_bang("bi", true).expect("bing");
    assert!(bing.url.contains("adlt=strict"));
}
