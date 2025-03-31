import generated.BUILD_GIT_COMMIT_SHA
import kotlinx.browser.document
import kotlinx.browser.window
import org.w3c.dom.HTMLInputElement
import org.w3c.dom.HTMLSpanElement
import org.w3c.dom.events.Event

fun triggerSearch() {
    val settings = overrideSettingsFromUrl(readSettingsFromCookie() ?: Settings.default)
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
        foundBang = findBangUrlByKey(settings.defaultBang)!!
    }

    val url = run {
        val ref = findReferral(settings, foundBang.url)
        val s = encodeURIComponent(query) + run {
            if (ref != null) "&${ref.referral}" else ""
        }
        foundBang.url.replace("{{{s}}}", s)
    }
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
        val browserId =
            document.getElementById("browser-id")?.let {
                (it as HTMLInputElement).value
            }.nonEmptyOrNull()

        Settings(defaultBang, bangChars, browserId).writeToCookie()
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
    document.getElementById("browser-id")?.let {
        val input = it as HTMLInputElement
        input.value = settings.browserId ?: ""
        input.addEventListener("input", { _: Event -> onSettingsChanged() })
    }

    document.getElementById("build-info")?.let {
        val span = it as HTMLSpanElement
        span.textContent = BUILD_GIT_COMMIT_SHA
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
