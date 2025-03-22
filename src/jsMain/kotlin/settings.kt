
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
)
