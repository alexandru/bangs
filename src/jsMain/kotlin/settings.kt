
val allBangs = arrayOf(
    // Brave
    Bang("https://search.brave.com/search?q={{{s}}}", "b"),
    Bang("https://search.brave.com/search?q=site:wikipedia.org%20{{{s}}}", "bwp"),
    Bang("https://search.brave.com/search?q=site:news.ycombinator.com%20{{{s}}}", "bhn"),
    Bang("https://search.brave.com/search?q=site:reddit.com%20{{{s}}}", "br"),
    Bang("https://search.brave.com/search?q=site:lobste.rs%20{{{s}}}", "blobsters"),
    Bang("https://search.brave.com/search?q=site:github.com%20{{{s}}}", "bgithub", "bgh"),
    Bang("https://search.brave.com/search?q=site:stackoverflow.com%20{{{s}}}", "bstackoverflow", "bso"),
    Bang("https://search.brave.com/search?q=site:social.alexn.org%20{{{s}}}", "bsocial"),
    Bang("https://search.brave.com/search?q=site:alexn.org%20{{{s}}}", "balexn"),
    // StartPage
    Bang("https://www.startpage.com/do/search?query={{{s}}}", "sp"),
    Bang("https://www.startpage.com/do/search?query=site:wikipedia.org%20{{{s}}}", "swp"),
    Bang("https://www.startpage.com/do/search?query=site:news.ycombinator.com%20{{{s}}}", "shn"),
    Bang("https://www.startpage.com/do/search?query=site:reddit.com%20{{{s}}}", "sr"),
    Bang("https://www.startpage.com/do/search?query=site:lobste.rs%20{{{s}}}", "slobsters"),
    Bang("https://www.startpage.com/do/search?query=site:github.com%20{{{s}}}", "sgithub", "sgh"),
    Bang("https://www.startpage.com/do/search?query=site:stackoverflow.com%20{{{s}}}", "sstackoverflow", "sso"),
    Bang("https://www.startpage.com/do/search?query=site:social.alexn.org%20{{{s}}}", "ssocial"),
    Bang("https://www.startpage.com/do/search?query=site:alexn.org%20{{{s}}}", "salexn"),
    // Qwant
    Bang("https://www.qwant.com/?q={{{s}}}", "q"),
    Bang("https://www.qwant.com/?q=site:wikipedia.org%20{{{s}}}", "qwp"),
    Bang("https://www.qwant.com/?q=site:news.ycombinator.com%20{{{s}}}", "qhn"),
    Bang("https://www.qwant.com/?q=site:reddit.com%20{{{s}}}", "qr"),
    Bang("https://www.qwant.com/?q=site:lobste.rs%20{{{s}}}", "qlobsters"),
    Bang("https://www.qwant.com/?q=site:github.com%20{{{s}}}", "qgithub", "qgh"),
    Bang("https://www.qwant.com/?q=site:stackoverflow.com%20{{{s}}}", "qstackoverflow", "qso"),
    Bang("https://www.qwant.com/?q=site:social.alexn.org%20{{{s}}}", "qsocial"),
    Bang("https://www.qwant.com/?q=site:alexn.org%20{{{s}}}", "qalexn"),
    // Google
    Bang("https://www.google.com/search?q={{{s}}}", "g", "google"),
    Bang("https://google.com/maps/place/{{{s}}}", "gm", "gmaps"),
    Bang("https://google.com/search?tbm=isch&q={{{s}}}&tbs=imgo:1", "gi", "gimages"),
    Bang("https://chrome.google.com/webstore/search/{{{s}}}?_category=extensions", "gws", "gwebstore"),
    Bang("https://www.youtube.com/results?search_query={{{s}}}", "youtube", "yt", "y"),
    Bang("https://www.google.com/search?q=site:wikipedia.org%20{{{s}}}", "gwp", "gwikipedia"),
    Bang("https://www.google.com/search?q=site:news.ycombinator.com%20{{{s}}}", "ghn"),
    Bang("https://www.google.com/search?q=site:reddit.com%20{{{s}}}", "gr", "greddit"),
    Bang("https://www.google.com/search?q=site:lobste.rs%20{{{s}}}", "globsters"),
    Bang("https://www.google.com/search?q=site:github.com%20{{{s}}}", "ggithub", "ggh"),
    Bang("https://www.google.com/search?q=site:stackoverflow.com%20{{{s}}}", "gstackoverflow", "gso"),
    Bang("https://www.google.com/search?q=site:social.alexn.org%20{{{s}}}", "gsocial"),
    Bang("https://www.google.com/search?q=site:alexn.org%20{{{s}}}", "galexn"),
    // Popular
    Bang("https://en.wikipedia.org/wiki/Special:Search?search={{{s}}}", "w", "wikipedia"),
    Bang("https://www.reddit.com/search?q={{{s}}}", "r", "reddit"),
    Bang("http://www.imdb.com/find?s=all&q={{{s}}}", "imdb"),
    Bang("https://hn.algolia.com/?q={{{s}}}", "hn"),
    Bang("https://lobste.rs/search?q={{{s}}}", "lobsters"),
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
    Bang.ctx(
        "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fkotlin",
        "kotlin",
        "kotlin", "kt"
    ),
)
