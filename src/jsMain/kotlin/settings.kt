data class Settings(
    val defaultBang: String,
    val defaultWebsite: String
)

val defaultSettings = Settings(
    defaultBang = "g",
    defaultWebsite = "https://www.google.com"
)
