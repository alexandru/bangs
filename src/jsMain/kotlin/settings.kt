const val DefaultBangDefault = "b"
const val Firefox = "firefox"
const val FirefoxMobile = "firefox-mobile"
const val Vivaldi = "vivaldi"

/**
 * I'd like my browser to get paid for my searches
 * (optional, if ID is in settings)
 */
val Referrals = arrayOf(
    // Firefox
    Referral(
        browserId = Firefox,
        hostname = "google.com",
        referral = "client=firefox-b-d"
    ),
    Referral(
        browserId = Firefox,
        hostname = "duckduckgo.com",
        referral = "t=ffab"
    ),
    // Firefox Mobile
    Referral(
        browserId = FirefoxMobile,
        hostname = "google.com",
        referral = "client=firefox-b-m"
    ),
    Referral(
        browserId = FirefoxMobile,
        hostname = "duckduckgo.com",
        referral = "t=ffab"
    ),
    // Vivaldi
    Referral(
        browserId = Vivaldi,
        hostname ="startpage.com",
        referral = "segment=startpage.vivaldi"
    ),
    Referral(
        browserId = Vivaldi,
        hostname = "qwant.com",
        referral = "client=brz-vivaldi&t=web"
    ),
    Referral(
        browserId = Vivaldi,
        hostname = "duckduckgo.com",
        referral = "t=vivaldi&ia=web"
    ),
)

/**
 * Limited queries that get combined with the [GeneralPurposeEngines] specified below.
 * E.g., `wp` becomes `gwp` for Google, `dwp` for DuckDuckGo, etc.
 */
val Queries = arrayOf(
    Query("site:wikipedia.org%20{{{s}}}", "w", "wp", "wiki", "wikipedia"),
    Query("site:news.ycombinator.com%20{{{s}}}", "hn", "hackernews"),
    Query("site:reddit.com%20{{{s}}}", "r", "reddit"),
    Query("site:lobste.rs%20{{{s}}}", "lobsters"),
    Query("site:github.com%20{{{s}}}", "github", "gh"),
    Query("site:gist.github.com%20{{{s}}}", "gist"),
    Query("site:stackoverflow.com%20{{{s}}}", "so"),
    Query("site:social.alexn.org%20{{{s}}}", "social"),
    Query("site:alexn.org%20{{{s}}}", "alexn"),
)

/**
 * General-purpose search engines. For specific ones, see [SpecialPurposeEngines].
 */
val GeneralPurposeEngines = arrayOf(
    // Google
    Bang("https://www.google.com/search?q={{{s}}}", "g", "google"),
    // Brave
    Bang("https://search.brave.com/search?q={{{s}}}", "b", "br"),
    // StartPage
    Bang("https://www.startpage.com/do/search?query={{{s}}}", "s", "sp"),
    // Qwant
    Bang("https://www.qwant.com/?q={{{s}}}", "q", "qw"),
    // DuckDuckGo
    Bang("https://duckduckgo.com/?q={{{s}}}", "d", "ddg"),
    // Bing
    Bang("https://www.bing.com/search?q={{{s}}}", "bi", "bing"),
)

/**
 * Safe search versions of [GeneralPurposeEngines].
 */
val SafeGeneralPurposeEngines = arrayOf(
    // Google
    Bang("https://www.google.com/search?q={{{s}}}&safe=active", "g", "google"),
    // Brave
    Bang("https://search.brave.com/search?q={{{s}}}&safesearch=strict", "b", "br"),
    // StartPage
    Bang("https://safe.startpage.com/do/search?query={{{s}}}", "s", "sp"),
    // Qwant
    Bang("https://www.qwant.com/?s=2&q={{{s}}}&safesearch=2", "q", "qw"),
    // DuckDuckGo
    Bang("https://safe.duckduckgo.com/?q={{{s}}}", "d", "ddg"),
    // Bing
    Bang("https://www.bing.com/search?q={{{s}}}&adlt=strict", "bi", "bing"),
)


val SpecialPurposeEngines = arrayOf(
    // Google
    Bang("https://google.com/maps/place/{{{s}}}", "gm", "gmaps"),
    Bang("https://google.com/search?tbm=isch&q={{{s}}}&tbs=imgo:1", "gi", "gimages"),
    Bang("https://chrome.google.com/webstore/search/{{{s}}}?_category=extensions", "gws", "gwebstore"),
    Bang("https://www.youtube.com/results?search_query={{{s}}}", "youtube", "yt", "y"),
    // Popular
    Bang("https://en.wikipedia.org/wiki/Special:Search?search={{{s}}}", "w", "wikipedia"),
    Bang("https://www.reddit.com/search?q={{{s}}}", "r", "reddit"),
    Bang("http://www.imdb.com/find?s=all&q={{{s}}}", "imdb"),
    Bang("https://hn.algolia.com/?q={{{s}}}", "hn"),
    Bang("https://lobste.rs/search?q={{{s}}}", "lobsters"),
    // github
    Bang("http://github.com/search?q={{{s}}}&type=Everything&repo=&langOverride=&start_value=1", "gh", "github"),
    Bang("https://github.com/search?utf8=%E2%9C%93&q={{{s}}}", "git"),
    Bang("https://gist.github.com/search?q={{{s}}}", "gist"),
    Bang("https://gist.github.com/search?q=user%3Aalexandru%20{{{s}}}", "gistm", "mygist"),
    // Local
    Bang("https://dexonline.ro/definitie/{{{s}}}", "dex", "dexonline"),
    // Programming
    Bang("https://index.scala-lang.org/search?q={{{s}}}", "scaladex", "scalai"),
    Bang("https://central.sonatype.com/search?q={{{s}}}", "maven", "mvn"),
    Bang("https://www.scala-lang.org/api/current/index.html?search={{{s}}}", "scalaapi"),
    Bang.ctx(
        "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fscala",
        "scala",
        "scala", "sc"
    ),
    Bang.ctx(
        "https://search.brave.com/goggles?q=json&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Fbrave%2Fgoggles-quickstart%2Fmain%2Fgoggles%2Frust_programming.goggle",
        "rust",
        "rust", "rs"
    ),
    Bang.ctx(
        "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fkotlin",
        "kotlin",
        "kotlin", "kt"
    ),
    // AI/LLM
    Bang("https://chatgpt.com/?hints=search&temporary-chat=true&prompt={{{s}}}", "gpt", "chatgpt", "ai", "aig"),
    Bang("https://chat.mistral.ai/chat?q={{{s}}}", "mistral", "aim"),
    Bang("https://www.perplexity.ai/search/?q={{{s}}}", "perplexity", "aip"),
)
