//! The engine registry: search engines, site queries, and referrals.

/// A resolved bang: the URL template (containing a `{{{s}}}` placeholder),
/// the keys it matched, and — for goggle-based bangs — the search context
/// word that replaces the bang token in the query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bang {
    pub url: String,
    pub keys: Vec<String>,
    pub search_context: Option<String>,
}

/// A static registry entry, converted into an owned [`Bang`] on lookup.
pub struct BangDef {
    pub url: &'static str,
    pub keys: &'static [&'static str],
    pub search_context: Option<&'static str>,
}

impl BangDef {
    /// Converts the static definition into an owned [`Bang`].
    fn to_bang(&self) -> Bang {
        Bang {
            url: self.url.to_string(),
            keys: self.keys.iter().map(|key| key.to_string()).collect(),
            search_context: self.search_context.map(|context| context.to_string()),
        }
    }
}

/// A site-restricted query template, combinable with general-purpose engine
/// keys: `wp` becomes `gwp` for Google, `dwp` for DuckDuckGo, and so on.
pub struct QueryDef {
    pub query: &'static str,
    pub keys: &'static [&'static str],
}

/// A referral tag appended to search URLs for a known browser.
#[derive(Debug, PartialEq)]
pub struct Referral {
    pub hostname: &'static str,
    pub browser_id: &'static str,
    pub referral: &'static str,
}

/// Fallback default bang key ("g" for Google), used when the settings'
/// own default bang key is unknown.
pub const DEFAULT_BANG_KEY: &str = "g";

/// Bangs that take precedence over the general-purpose engines.
pub const SPECIAL_PURPOSE_ENGINES: &[BangDef] = &[
    // Google
    BangDef {
        url: "https://google.com/maps/place/{{{s}}}",
        keys: &["gm", "gmaps"],
        search_context: None,
    },
    BangDef {
        url: "https://google.com/search?tbm=isch&q={{{s}}}&tbs=imgo:1",
        keys: &["gi", "gimages"],
        search_context: None,
    },
    BangDef {
        url: "https://chrome.google.com/webstore/search/{{{s}}}?_category=extensions",
        keys: &["gws", "gwebstore"],
        search_context: None,
    },
    BangDef {
        url: "https://www.youtube.com/results?search_query={{{s}}}",
        keys: &["youtube", "yt", "y"],
        search_context: None,
    },
    // Popular
    BangDef {
        url: "https://en.wikipedia.org/wiki/Special:Search?search={{{s}}}",
        keys: &["w", "wikipedia"],
        search_context: None,
    },
    BangDef {
        url: "https://www.reddit.com/search?q={{{s}}}",
        keys: &["r", "reddit"],
        search_context: None,
    },
    BangDef {
        url: "http://www.imdb.com/find?s=all&q={{{s}}}",
        keys: &["imdb"],
        search_context: None,
    },
    BangDef {
        url: "https://hn.algolia.com/?q={{{s}}}",
        keys: &["hn"],
        search_context: None,
    },
    BangDef {
        url: "https://lobste.rs/search?q={{{s}}}",
        keys: &["lobsters"],
        search_context: None,
    },
    // github
    BangDef {
        url: "http://github.com/search?q={{{s}}}&type=Everything&repo=&langOverride=&start_value=1",
        keys: &["gh", "github"],
        search_context: None,
    },
    BangDef {
        url: "https://github.com/search?utf8=%E2%9C%93&q={{{s}}}",
        keys: &["git"],
        search_context: None,
    },
    BangDef {
        url: "https://gist.github.com/search?q={{{s}}}",
        keys: &["gist"],
        search_context: None,
    },
    BangDef {
        url: "https://gist.github.com/search?q=user%3Aalexandru%20{{{s}}}",
        keys: &["gistm", "mygist"],
        search_context: None,
    },
    // Local
    BangDef {
        url: "https://dexonline.ro/definitie/{{{s}}}",
        keys: &["dex", "dexonline"],
        search_context: None,
    },
    // Programming
    BangDef {
        url: "https://index.scala-lang.org/search?q={{{s}}}",
        keys: &["scaladex", "scalai"],
        search_context: None,
    },
    BangDef {
        url: "https://central.sonatype.com/search?q={{{s}}}",
        keys: &["maven", "mvn"],
        search_context: None,
    },
    BangDef {
        url: "https://www.scala-lang.org/api/current/index.html?search={{{s}}}",
        keys: &["scalaapi"],
        search_context: None,
    },
    BangDef {
        url: "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fscala",
        keys: &["scala", "sc"],
        search_context: Some("scala"),
    },
    BangDef {
        url: "https://search.brave.com/goggles?q=json&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Fbrave%2Fgoggles-quickstart%2Fmain%2Fgoggles%2Frust_programming.goggle",
        keys: &["rust", "rs"],
        search_context: Some("rust"),
    },
    BangDef {
        url: "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fkotlin",
        keys: &["kotlin", "kt"],
        search_context: Some("kotlin"),
    },
    // AI/LLM
    BangDef {
        url: "https://chatgpt.com/?hints=search&temporary-chat=true&prompt={{{s}}}",
        keys: &["gpt", "chatgpt", "ai", "aig"],
        search_context: None,
    },
    BangDef {
        url: "https://chat.mistral.ai/chat?q={{{s}}}",
        keys: &["mistral", "aim"],
        search_context: None,
    },
    BangDef {
        url: "https://www.perplexity.ai/search/?q={{{s}}}",
        keys: &["perplexity", "aip"],
        search_context: None,
    },
];

/// General-purpose search engines; see [`SPECIAL_PURPOSE_ENGINES`] for the
/// specific ones.
pub const GENERAL_PURPOSE_ENGINES: &[BangDef] = &[
    BangDef {
        url: "https://www.google.com/search?q={{{s}}}",
        keys: &["g", "google"],
        search_context: None,
    },
    BangDef {
        url: "https://search.brave.com/search?q={{{s}}}",
        keys: &["b", "br"],
        search_context: None,
    },
    BangDef {
        url: "https://www.startpage.com/do/search?query={{{s}}}",
        keys: &["s", "sp"],
        search_context: None,
    },
    BangDef {
        url: "https://www.qwant.com/?q={{{s}}}",
        keys: &["q", "qw"],
        search_context: None,
    },
    BangDef {
        url: "https://duckduckgo.com/?q={{{s}}}",
        keys: &["d", "ddg"],
        search_context: None,
    },
    BangDef {
        url: "https://www.bing.com/search?q={{{s}}}",
        keys: &["bi", "bing"],
        search_context: None,
    },
    BangDef {
        url: "https://marginalia-search.com/search?query={{{s}}}",
        keys: &["ma"],
        search_context: None,
    },
    BangDef {
        url: "https://www.mojeek.com/search?q={{{s}}}",
        keys: &["mo"],
        search_context: None,
    },
];

/// Safe-search variants of [`GENERAL_PURPOSE_ENGINES`].
pub const SAFE_GENERAL_PURPOSE_ENGINES: &[BangDef] = &[
    BangDef {
        url: "https://www.google.com/search?q={{{s}}}&safe=active",
        keys: &["g", "google"],
        search_context: None,
    },
    BangDef {
        url: "https://safe.search.brave.com/search?q={{{s}}}",
        keys: &["b", "br"],
        search_context: None,
    },
    BangDef {
        url: "https://safe.startpage.com/do/search?query={{{s}}}",
        keys: &["s", "sp"],
        search_context: None,
    },
    BangDef {
        url: "https://www.qwant.com/?s=2&q={{{s}}}&safesearch=2",
        keys: &["q", "qw"],
        search_context: None,
    },
    BangDef {
        url: "https://safe.duckduckgo.com/?q={{{s}}}",
        keys: &["d", "ddg"],
        search_context: None,
    },
    BangDef {
        url: "https://www.bing.com/search?q={{{s}}}&adlt=strict",
        keys: &["bi", "bing"],
        search_context: None,
    },
    BangDef {
        url: "https://marginalia-search.com/search?query={{{s}}}&nsfw=smut",
        keys: &["ma"],
        search_context: None,
    },
    BangDef {
        url: "https://www.mojeek.com/search?q={{{s}}}&safe=1",
        keys: &["mo"],
        search_context: None,
    },
];

/// Site queries combinable with [`GENERAL_PURPOSE_ENGINES`] keys.
pub const QUERIES: &[QueryDef] = &[
    QueryDef {
        query: "site:wikipedia.org%20{{{s}}}",
        keys: &["w", "wp", "wiki", "wikipedia"],
    },
    QueryDef {
        query: "site:news.ycombinator.com%20{{{s}}}",
        keys: &["hn", "hackernews"],
    },
    QueryDef {
        query: "site:reddit.com%20{{{s}}}",
        keys: &["r", "reddit"],
    },
    QueryDef {
        query: "site:lobste.rs%20{{{s}}}",
        keys: &["lobsters"],
    },
    QueryDef {
        query: "site:github.com%20{{{s}}}",
        keys: &["github", "gh"],
    },
    QueryDef {
        query: "site:gist.github.com%20{{{s}}}",
        keys: &["gist"],
    },
    QueryDef {
        query: "site:stackoverflow.com%20{{{s}}}",
        keys: &["so"],
    },
    QueryDef {
        query: "site:social.alexn.org%20{{{s}}}",
        keys: &["social"],
    },
    QueryDef {
        query: "site:alexn.org%20{{{s}}}",
        keys: &["alexn"],
    },
];

/// Referral tags for browsers that get paid for the searches (optional,
/// only used when a browser id is set in the settings).
pub const REFERRALS: &[Referral] = &[
    // Firefox
    Referral {
        browser_id: "firefox",
        hostname: "google.com",
        referral: "client=firefox-b-d",
    },
    Referral {
        browser_id: "firefox",
        hostname: "duckduckgo.com",
        referral: "t=ffab",
    },
    // Firefox Mobile
    Referral {
        browser_id: "firefox-mobile",
        hostname: "google.com",
        referral: "client=firefox-b-m",
    },
    Referral {
        browser_id: "firefox-mobile",
        hostname: "duckduckgo.com",
        referral: "t=ffab",
    },
    // Vivaldi
    Referral {
        browser_id: "vivaldi",
        hostname: "startpage.com",
        referral: "segment=startpage.vivaldi",
    },
    Referral {
        browser_id: "vivaldi",
        hostname: "qwant.com",
        referral: "client=brz-vivaldi&t=web",
    },
    Referral {
        browser_id: "vivaldi",
        hostname: "duckduckgo.com",
        referral: "t=vivaldi&ia=web",
    },
];

/// Resolves a bang key to an engine. Special-purpose engines match exactly
/// and take precedence; general-purpose engines then match exactly, or by
/// prefix when the remainder is a known site query (`gw` = Google +
/// Wikipedia). `safe` selects the safe-search engine variants.
#[must_use]
pub fn find_bang(key: &str, safe: bool) -> Option<Bang> {
    for bang in SPECIAL_PURPOSE_ENGINES {
        if bang.keys.contains(&key) {
            return Some(bang.to_bang());
        }
    }
    let engines = if safe {
        SAFE_GENERAL_PURPOSE_ENGINES
    } else {
        GENERAL_PURPOSE_ENGINES
    };
    for engine in engines {
        for engine_key in engine.keys {
            if *engine_key == key {
                return Some(engine.to_bang());
            }
            if key.starts_with(engine_key) {
                // Sub-key lookup: "gw" = "g" + "w" (Google + Wikipedia).
                let subkey = &key[engine_key.len()..];
                for query in QUERIES {
                    if query.keys.contains(&subkey) {
                        return Some(Bang {
                            url: engine.url.replace("{{{s}}}", query.query),
                            keys: vec![format!("{engine_key}{subkey}")],
                            search_context: None,
                        });
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
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
}
