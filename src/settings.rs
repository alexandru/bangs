//! The engine registry: the available referrals, site queries, and search
//! engines.

use crate::models::{BangDef, QueryDef, Referral};

/// Fallback default bang key ("g" for Google), used when the settings'
/// own default bang key is unknown.
pub const DEFAULT_BANG_KEY: &str = "g";

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
