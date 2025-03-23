
data class Settings(
    val defaultBang: String,
    val bangChars: String,
) {
    companion object{
        val default = Settings(
            defaultBang = "g",
            bangChars = "!@/"
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
