data class Settings(
    val defaultBang: String,
    val defaultWebsite: String
)

data class Bang(
    val url: String,
    val keys: Array<out String>
) {
    companion object {
        operator fun invoke(url: String, vararg keys: String) =
            Bang(url, keys)
    }
}

val defaultSettings = Settings(
    defaultBang = "g",
    defaultWebsite = "https://www.google.com"
)

val allBangs = arrayOf(
    // Brave
    Bang("https://search.brave.com/search?q={{{s}}}", "b"),
    Bang("https://search.brave.com/search?q=site:wikipedia.org%20{{{s}}}", "bwp"),
    Bang("https://search.brave.com/search?q=site:news.ycombinator.com%20{{{s}}}", "bhn"),
    Bang("https://search.brave.com/search?q=site:reddit.com%20{{{s}}}", "br"),
    // Google
    Bang("https://www.google.com/search?q={{{s}}}", "g", "google"),
    Bang("https://google.com/maps/place/{{{s}}}", "gm", "gmaps"),
    Bang("https://google.com/search?tbm=isch&q={{{s}}}&tbs=imgo:1", "gi", "gimages"),
    Bang("https://www.google.com/search?q=site:wikipedia.org%20{{{s}}}", "gwp", "gwikipedia"),
    Bang("https://www.google.com/search?q=site:news.ycombinator.com%20{{{s}}}", "ghn"),
    Bang("https://www.google.com/search?q=site:reddit.com%20{{{s}}}", "gr", "greddit"),
    Bang("https://chrome.google.com/webstore/search/{{{s}}}?_category=extensions", "gws", "gwebstore"),
    Bang("https://www.youtube.com/results?search_query={{{s}}}", "youtube", "yt", "y"),
    // Popular
    Bang("https://en.wikipedia.org/wiki/Special:Search?search={{{s}}}", "w", "wikipedia"),
    Bang("https://www.reddit.com/search?q={{{s}}}", "r", "reddit"),
    Bang("http://www.imdb.com/find?s=all&q={{{s}}}", "imdb"),
    Bang("https://hn.algolia.com/?q={{{s}}}", "hn"),
    // github
    Bang("http://github.com/search?q={{{s}}}&type=Everything&repo=&langOverride=&start_value=1", "gh", "github"),
    Bang("https://github.com/search?utf8=%E2%9C%93&q={{{s}}}", "git"),
    // Local
    Bang("https://dexonline.ro/definitie/{{{s}}}", "dex", "dexonline"),
)
