import kotlin.test.Test
import kotlin.test.assertEquals

class ParserTest {
    @Test
    fun testParser() {
        val v1 = extractBangsFromQuery("hn hello world").toList()
        assertEquals(listOf(), v1)

        val v2 = extractBangsFromQuery("hn hello world !g").toList()
        assertEquals(listOf("g"), v2)

        val v3 = extractBangsFromQuery("hn hello !w world !g").toList()
        assertEquals(listOf("g", "w"), v3)

        val v4 = extractBangsFromQuery("best json library !scala").toList()
        assertEquals(listOf("scala"), v4)

        val v5 = extractBangsFromQuery("blah blah !scalaapi").toList()
        assertEquals(listOf("scalaapi"), v5)
    }

    @Test
    fun testRemoveBangFromQuery() {
        val v1 = removeBangFromQuery("hn hello world !g", "g", null)
        assertEquals("hn hello world", v1)

        val v2 = removeBangFromQuery("hn hello !w world !g", "w", null)
        assertEquals("hn hello world !g", v2)

        val v3 = removeBangFromQuery("hn hello !w world !g", "g", null)
        assertEquals("hn hello !w world", v3)

        val v4 = removeBangFromQuery("!hn hello !w world !g", "hn", null)
        assertEquals("hello !w world !g", v4)

        val v5 = removeBangFromQuery("best json library !scala", "scala", "scala")
        assertEquals("best json library scala", v5)

        val v6 = removeBangFromQuery("!scala best json library", "scala", "scala")
        assertEquals("scala best json library", v6)

        val v7 = removeBangFromQuery("best !scala json library", "scala", "scala")
        assertEquals("best scala json library", v7)
    }
}
