import kotlinx.browser.window

external fun decodeURIComponent(encodedURI: String): String

fun getQueryParameter(name: String): String? {
    val url = window.location.search
    val regex = Regex("""[&?]${name}=([^&]*)""")
    val encodedValue = regex.find(url)?.groupValues?.get(1)
    return encodedValue?.let { decodeURIComponent(it).replace("+", " ") }
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
    val ch = key.getOrNull(0) ?: return null
    val bangsData = when {
        ch.isDigit() -> bangsData.getOrNull(ch - '0') ?: return null
        ch.isLetter() -> bangsData.getOrNull(ch - 'a' + 10) ?: return null
        else -> return null
    }

    for (element in bangsData) {
        if (element.first == key)
            return element.second
    }
    return null
}

fun removeBangFromQuery(query: String, bang: String): String {
    return query
        .replaceLastRegex("(^|\\s+|\\b)[!]${Regex.escape(bang)}($|\\s+|\\b)", " ")
        .trim()
}


fun redirectToUrl(url: String) {
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
    val rawQuery = getQueryParameter("q")
    if (rawQuery == null || rawQuery.isEmpty()) {
        redirectToUrl(defaultSettings.defaultWebsite)
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
    redirectToUrl(url)
}
