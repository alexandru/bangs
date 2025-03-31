data class Settings(
    val defaultBang: String,
    val bangChars: String,
    val browserId: String?
) {
    companion object{
        val default = Settings(
            defaultBang = "g",
            bangChars = "!@/",
            browserId = null
        )
    }
}

data class Bang(
    val url: String,
    val keys: List<String>,
    val searchContext: String?,
) {
    companion object {
        operator fun invoke(url: String, vararg keys: String) =
            Bang(url, keys.toList(), searchContext = null)

        fun ctx(url: String, searchContext: String, vararg keys: String) =
            Bang(url, keys.toList(), searchContext)
    }
}

data class Query(
    val query: String,
    val keys: List<String>,
) {
    companion object {
        operator fun invoke(query: String, vararg keys: String) =
            Query(query, keys.toList())
    }
}

data class Referral(
    val hostname: String,
    val browserId: String,
    val referral: String
)
