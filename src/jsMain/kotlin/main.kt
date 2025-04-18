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
        foundBang = findBangUrlByKey(settings.defaultBang) ?:
                findBangUrlByKey(DefaultBangDefault)!!
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

    // Initialize search form
    val form = document.getElementById("search-form")
    val input = document.getElementById("search-input") as HTMLInputElement
    form?.addEventListener("submit", { e: Event ->
        if (input.value.trim().isNotEmpty()) {
            e.preventDefault()
            window.location.href = "/search/#q=" + encodeURIComponent(input.value)
        }
    })

    // Initialize settings form
    val settings = readSettingsFromCookie() ?: Settings.default
    document.getElementById("default-bang")?.let {
        val fi = it as HTMLInputElement
        fi.value = settings.defaultBang
        fi.addEventListener("input", { _: Event -> onSettingsChanged() })
    }
    document.getElementById("bang-chars")?.let {
        val fi = it as HTMLInputElement
        fi.value = settings.bangChars
        fi.addEventListener("input", { _: Event -> onSettingsChanged() })
    }
    document.getElementById("browser-id")?.let {
        val fi = it as HTMLInputElement
        fi.value = settings.browserId ?: ""
        fi.addEventListener("input", { _: Event -> onSettingsChanged() })
    }

    // Initialize build info
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
