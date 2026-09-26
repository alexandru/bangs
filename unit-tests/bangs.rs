use super::*;

#[test]
fn find_bang_resolves_simple_engines() -> Result<(), &'static str> {
    let goog = find_bang("g", false).ok_or("google")?;
    assert!(goog.keys.iter().any(|key| key == "g"));
    assert!(goog.url.contains("google.com"));

    let brave = find_bang("br", false).ok_or("brave")?;
    assert!(brave.keys.iter().any(|key| key == "br"));
    assert!(brave.url.contains("search.brave.com"));

    let github = find_bang("gh", false).ok_or("github")?;
    assert!(github.keys.iter().any(|key| key == "gh"));
    assert!(github.url.contains("github.com"));
    Ok(())
}

#[test]
fn find_bang_synthesizes_engine_plus_site_query() -> Result<(), &'static str> {
    let wikipedia_by_google = find_bang("gw", false).ok_or("gw")?;
    assert!(wikipedia_by_google.keys.iter().any(|key| key == "gw"));
    assert_eq!(
        wikipedia_by_google.url,
        "https://www.google.com/search?q=site:wikipedia.org%20{{{s}}}"
    );

    let hn_by_google = find_bang("ghn", false).ok_or("ghn")?;
    assert_eq!(hn_by_google.keys, vec!["ghn".to_string()]);
    assert!(hn_by_google.url.contains("news.ycombinator.com"));
    Ok(())
}

#[test]
fn find_bang_prefers_special_purpose_engines() -> Result<(), &'static str> {
    // "w" is Wikipedia directly, not Google + Wikipedia.
    let wikipedia = find_bang("w", false).ok_or("w")?;
    assert!(wikipedia.url.contains("en.wikipedia.org"));

    let scala_goggles = find_bang("scala", false).ok_or("scala")?;
    assert_eq!(scala_goggles.search_context, Some("scala".to_string()));
    Ok(())
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
fn find_bang_resolves_safe_engine_variants() -> Result<(), &'static str> {
    let goog = find_bang("g", true).ok_or("google")?;
    assert!(goog.url.contains("safe=active"));
    let brave = find_bang("br", true).ok_or("brave")?;
    assert!(brave.url.contains("safe.search.brave.com"));
    let duck = find_bang("d", true).ok_or("duckduckgo")?;
    assert!(duck.url.contains("safe.duckduckgo.com"));
    let startpage = find_bang("s", true).ok_or("startpage")?;
    assert!(startpage.url.contains("safe.startpage.com"));
    let qwant = find_bang("q", true).ok_or("qwant")?;
    assert!(qwant.url.contains("safesearch=2"));
    let bing = find_bang("bi", true).ok_or("bing")?;
    assert!(bing.url.contains("adlt=strict"));
    Ok(())
}
