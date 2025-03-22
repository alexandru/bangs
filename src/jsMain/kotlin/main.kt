@file:OptIn(ExperimentalJsExport::class)

import kotlinx.browser.window

fun triggerSearch() {
    val debug = getQueryParameter("debug") != null
    val rawQuery = getQueryParameter("q")
    if (rawQuery == null || rawQuery.isEmpty()) {
        return redirectToUrl(window.location.origin, debug)
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

@JsExport
fun saveSettingsForm(defaultBang: String, bangChars: String) {
    Settings(defaultBang, bangChars).writeToCookie()
}

fun main() {
    // /search/ page
    if (window.location.pathname.startsWith("/search/")) {
        return triggerSearch()
    }
}
