import kotlinx.browser.document
import kotlinx.browser.window
import org.w3c.dom.HTMLInputElement
import org.w3c.dom.events.Event

fun triggerSearch() {
    val settings = readSettingsFromCookie() ?: Settings.default
    val debug = getQueryParameter("debug") != null
    val rawQuery = getQueryParameter("q")
    if (rawQuery == null || rawQuery.isEmpty()) {
        return redirectToUrl(window.location.origin, debug)
    }

    val bangs = extractBangsFromQuery(rawQuery, settings)
    var foundBang: Bang? = null
    var query: String = rawQuery

    for (bang in bangs) {
        foundBang = findBangUrlByKey(bang)
        if (foundBang != null) {
            query = removeBangFromQuery(
                rawQuery,
                bang,
                foundBang.searchContext,
                settings
            )
            break
        }
    }

    if (foundBang == null) {
        // Defaults to Google
        foundBang = findBangUrlByKey(settings.defaultBang)!!
    }

    val url = foundBang.url.replace("{{{s}}}", encodeURIComponent(query))
    redirectToUrl(url, debug)
}

fun initHomePage() {
    fun onSettingsChanged() {
        val defaultBang =
            document.getElementById("default-bang")?.let {
                (it as HTMLInputElement).value
            } ?: Settings.default.defaultBang
        val bangChars =
            document.getElementById("bang-chars")?.let {
                (it as HTMLInputElement).value
            } ?: Settings.default.bangChars

        Settings(defaultBang, bangChars).writeToCookie()
    }

    val settings = readSettingsFromCookie() ?: Settings.default
    document.getElementById("default-bang")?.let {
        val input = it as HTMLInputElement
        input.value = settings.defaultBang
        input.addEventListener("input", { _: Event -> onSettingsChanged() })
    }
    document.getElementById("bang-chars")?.let {
        val input = it as HTMLInputElement
        input.value = settings.bangChars
        input.addEventListener("input", { _: Event -> onSettingsChanged() })
    }
}

fun main() {
    if (window.location.pathname.startsWith("/search/")) {
        return triggerSearch()
    } else {
        document.addEventListener("DOMContentLoaded", { _: Event ->
            initHomePage()
        })
    }
}
