import kotlinx.browser.window

external fun decodeURIComponent(encodedURI: String): String
external fun encodeURIComponent(uriComponent: String): String

fun getQueryParameter(name: String): String? {
    fun fromString(url: String): String? {
        val regex = Regex("""[&?#]${name}=([^&]*)""")
        val encodedValue = regex.find(url)?.groupValues?.get(1)
        return encodedValue?.let {
            decodeURIComponent(it).replace("+", " ")
        }
    }
    return fromString(window.location.search) ?: fromString(window.location.hash)
}

fun extractBangsFromQuery(rawQuery: String, bangChar: String = "!"): Array<String> {
    val parts = rawQuery.split("\\s+".toRegex())
    val bangs = ArrayList<String>()

    for (i in parts.indices) {
        if (parts[i].startsWith(bangChar)) {
            bangs.add(parts[i].substring(1))
        }
    }
    return bangs.reversed().toTypedArray()
}

fun findBangUrlByKey(key: String): String? {
    for (bang in allBangs) {
        for (bangKey in bang.keys) {
            if (bangKey == key) {
                return bang.url
            }
        }
    }
    return null
}

fun removeBangFromQuery(query: String, bang: String): String {
    return query
        .replaceLastRegex("(^|\\s+|\\b)[!]${Regex.escape(bang)}($|\\s+|\\b)", " ")
        .trim()
}


fun redirectToUrl(url: String, debug: Boolean) {
    if (debug)
        throw Exception("Redirect to $url")
    window.location.replace(url)
}

fun String.replaceLastRegex(regex: String, replacement: String): String {
    val pattern = regex.toRegex().findAll(this)
    // Find the last match
    var lastOccurrence = IntRange(-1, -1)
    for (match in pattern) {
        lastOccurrence = match.range
    }
    if (lastOccurrence.first == -1) {
        return this
    }
    return this.replaceRange(lastOccurrence, replacement)
}

fun main() {
    val debug = getQueryParameter("debug") != null
    val rawQuery = getQueryParameter("q")
    if (rawQuery == null || rawQuery.isEmpty()) {
        redirectToUrl(defaultSettings.defaultWebsite, debug)
        return
    }

    val bangs = extractBangsFromQuery(rawQuery)
    var urlTemplate: String? = null
    var query: String = rawQuery

    for (bang in bangs) {
        urlTemplate = findBangUrlByKey(bang)
        if (urlTemplate != null) {
            query = removeBangFromQuery(rawQuery, bang)
            break
        }
    }

    if (urlTemplate == null) {
        // Defaults to Google
        urlTemplate = findBangUrlByKey(defaultSettings.defaultBang)!!
    }

    val url = urlTemplate.replace("{{{s}}}", encodeURIComponent(query))
    redirectToUrl(url, debug)
}
