import kotlin.test.Test
import kotlin.test.assertEquals

class BrowserTest {
    @Test
    fun readAndWriteCookie() {
        writeCookie("test", "value", 1)
        assertEquals("value", readCookie("test"))
    }

    @Test
    fun readAndWriteSettings() {
        val settings = Settings(
            defaultBang = "blah",
            bangChars = "!@/"
        )
        settings.writeToCookie()
        val readSettings = readSettingsFromCookie()
        assertEquals(settings, readSettings)
    }
}
