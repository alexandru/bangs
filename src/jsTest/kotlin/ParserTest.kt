import kotlin.test.Test
import kotlin.test.assertEquals

class ParserTest {
    @Test
    fun testParser() {
        val v1 = extractBangsFromQuery("hn hello world").toList()
        assertEquals(v1, listOf())

        val v2 = extractBangsFromQuery("hn hello world !g").toList()
        assertEquals(v2, listOf("g"))

        val v3 = extractBangsFromQuery("hn hello !w world !g").toList()
        assertEquals(v3, listOf("g", "w"))
    }

    @Test
    fun testRemoveBangFromQuery() {
        val v1 = removeBangFromQuery("hn hello world !g", "g")
        assertEquals(v1, "hn hello world")

        val v2 = removeBangFromQuery("hn hello !w world !g", "w")
        assertEquals(v2, "hn hello world !g")

        val v3 = removeBangFromQuery("hn hello !w world !g", "g")
        assertEquals(v3, "hn hello !w world")

        val v4 = removeBangFromQuery("!hn hello !w world !g", "hn")
        assertEquals(v4, "hello !w world !g")
    }
}
