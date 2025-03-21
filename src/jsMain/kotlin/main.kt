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

fun extractBangsFromQuery(rawQuery: String): Array<String> {
    val parts = rawQuery.split("\\s+".toRegex())
    val bangs = ArrayList<String>()

    for (i in parts.indices) {
        if (parts[i].isNotEmpty() && defaultSettings.bangChars.contains(parts[i].take(1))) {
            bangs.add(parts[i].substring(1))
        }
    }
    return bangs.reversed().toTypedArray()
}

fun findBangUrlByKey(key: String): Bang? {
    for (bang in allBangs) {
        for (bangKey in bang.keys) {
            if (bangKey == key) {
                return bang
            }
        }
    }
    return null
}

fun removeBangFromQuery(query: String, bang: String, replacement: String?): String {
    val regex =
        if (replacement == null)
            "(^|\\s+|\\b)[${defaultSettings.bangChars}]${Regex.escape(bang)}($|\\s+|\\b)"
        else
            "(?<=^|\\b|\\s+)[${defaultSettings.bangChars}]${Regex.escape(bang)}(?=$|\\b|\\s+)"

    return query
        .replaceLastRegex(regex, replacement ?: " ")
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
    var foundBang: Bang? = null
    var query: String = rawQuery

    for (bang in bangs) {
        foundBang = findBangUrlByKey(bang)
        if (foundBang != null) {
            query = removeBangFromQuery(rawQuery, bang, foundBang.searchContext)
            break
        }
    }

    if (foundBang == null) {
        // Defaults to Google
        foundBang = findBangUrlByKey(defaultSettings.defaultBang)!!
    }

    val url = foundBang.url.replace("{{{s}}}", encodeURIComponent(query))
    redirectToUrl(url, debug)
}
