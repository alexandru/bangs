import kotlin.js.Date
import kotlinx.browser.document
import kotlinx.browser.window

external fun decodeURIComponent(encodedURI: String): String
external fun encodeURIComponent(uriComponent: String): String

fun readCookie(name: String): String? {
    val cookieString = document.cookie
    val regex = Regex("""${name}=([^;]+)""")
    val match = regex.find(cookieString)
    return match?.groupValues?.get(1)?.let { decodeURIComponent(it) }
}

fun writeCookie(name: String, value: String, daysUntilExpire: Int) {
    val encodedValue = encodeURIComponent(value)
    val expires = Date(Date.now() + (daysUntilExpire * 24 * 60 * 60 * 1000))
    val expiresString = "expires=${expires.toUTCString()}"
    document.cookie = "$name=$encodedValue; $expiresString; path=/; domain=${window.location.hostname}"
}

fun readSettingsFromCookie(): Settings? {
    val jsonStr = readCookie("settings") ?: return null
    console.log("Restoring settings: ", jsonStr)
    val json = JSON.parse<dynamic>(jsonStr)
    return Settings(
        defaultBang = (json["defaultBang"] as String?).nonEmptyOrNull() ?: Settings.default.defaultBang,
        bangChars = (json["bangChars"] as String?).nonEmptyOrNull() ?: Settings.default.bangChars
    )
}

fun Settings.writeToCookie() {
    val dict = js("{}")
    dict["defaultBang"] = this.defaultBang
    dict["bangChars"] = this.bangChars
    val json = JSON.stringify(dict)
    console.log("Saving settings: ", json)
    writeCookie("settings", json, daysUntilExpire = 365 * 10)
}

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

fun extractBangsFromQuery(
    rawQuery: String,
    settings: Settings
): Array<String> {
    val parts = rawQuery.split("\\s+".toRegex())
    val bangs = ArrayList<String>()

    for (i in parts.indices) {
        if (parts[i].isNotEmpty() && settings.bangChars.contains(parts[i].take(1))) {
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

fun removeBangFromQuery(
    query: String,
    bang: String,
    replacement: String?,
    defaultSettings: Settings
): String {
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

fun String?.nonEmptyOrNull(): String? {
    return if (this.isNullOrEmpty()) null else this
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

