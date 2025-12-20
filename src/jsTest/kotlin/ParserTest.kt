import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class ParserTest {
    @Test
    fun testParser() {
        val v1 = extractBangsFromQuery("hn hello world", Settings.default).toList()
        assertEquals(listOf(), v1)

        val v2 = extractBangsFromQuery("hn hello world !g", Settings.default).toList()
        assertEquals(listOf("g"), v2)

        val v3 = extractBangsFromQuery("hn hello !w world !g", Settings.default).toList()
        assertEquals(listOf("g", "w"), v3)

        val v4 = extractBangsFromQuery("best json library !scala", Settings.default).toList()
        assertEquals(listOf("scala"), v4)

        val v5 = extractBangsFromQuery("blah blah !scalaapi", Settings.default).toList()
        assertEquals(listOf("scalaapi"), v5)
    }

    @Test
    fun testRemoveBangFromQuery() {
        val v1 = removeBangFromQuery("hn hello world !g", "g", null, Settings.default)
        assertEquals("hn hello world", v1)

        val v2 = removeBangFromQuery("hn hello !w world !g", "w", null, Settings.default)
        assertEquals("hn hello world !g", v2)

        val v3 = removeBangFromQuery("hn hello !w world !g", "g", null, Settings.default)
        assertEquals("hn hello !w world", v3)

        val v4 = removeBangFromQuery("!hn hello !w world !g", "hn", null, Settings.default)
        assertEquals("hello !w world !g", v4)

        val v5 = removeBangFromQuery("best json library !scala", "scala", "scala", Settings.default)
        assertEquals("best json library scala", v5)

        val v6 = removeBangFromQuery("!scala best json library", "scala", "scala", Settings.default)
        assertEquals("scala best json library", v6)

        val v7 = removeBangFromQuery("best !scala json library", "scala", "scala", Settings.default)
        assertEquals("best scala json library", v7)
    }

    @Test
    fun testFindSimpleEngineBangs() {
        val goog = findBangUrlByKey("g", safe=false)
        assertTrue("find google (1)") {
            goog?.keys?.contains("g") ?: false
        }
        assertTrue("find google (2)") {
            goog?.url?.contains("google.com") ?: false
        }

        val brave = findBangUrlByKey("br", safe=false)
        assertTrue("find brave (1)") {
            brave?.keys?.contains("br") ?: false
        }
        assertTrue("find brave (2)") {
            brave?.url?.contains("search.brave.com") ?: false
        }

        val github = findBangUrlByKey("gh", safe=false)
        assertTrue("find github (1)") {
            github?.keys?.contains("gh") ?: false
        }
        assertTrue("find github (2)") {
            github?.url?.contains("github.com") ?: false
        }

        val wikipediaByGoogle = findBangUrlByKey("gw", safe=false)
        assertTrue("find wikipedia by google (1)") {
            wikipediaByGoogle?.keys?.contains("gw") ?: false
        }
        assertTrue("find wikipedia by google (2)") {
            wikipediaByGoogle?.url?.contains("google.com") ?: false
        }
        assertTrue("find wikipedia by google (3)") {
            wikipediaByGoogle?.url?.contains("wikipedia.org") ?: false
        }
    }

    @Test
    fun testFindSimpleEngineBangsSafe() {
        val googSafe = findBangUrlByKey("g", safe=true)
        assertTrue("find safe google (key)") {
            googSafe?.keys?.contains("g") ?: false
        }
        assertTrue("find safe google (url)") {
            googSafe?.url?.contains("safe=active") ?: false
        }

        val braveSafe = findBangUrlByKey("br", safe=true)
        assertTrue("find safe brave (key)") {
            braveSafe?.keys?.contains("br") ?: false
        }
        assertTrue("find safe brave (url)") {
            braveSafe?.url?.contains("safe.search.brave.com") ?: false
        }

        val duckSafe = findBangUrlByKey("d", safe=true)
        assertTrue("find safe duckduckgo (key)") {
            duckSafe?.keys?.contains("d") ?: false
        }
        assertTrue("find safe duckduckgo (url)") {
            duckSafe?.url?.contains("safe.duckduckgo.com") ?: false
        }

        val startPageSafe = findBangUrlByKey("s", safe=true)
        assertTrue("find safe startpage (key)") {
            startPageSafe?.keys?.contains("s") ?: false
        }
        assertTrue("find safe startpage (url)") {
            startPageSafe?.url?.contains("safe.startpage.com") ?: false
        }

        val qwantSafe = findBangUrlByKey("q", safe=true)
        assertTrue("find safe qwant (key)") {
            qwantSafe?.keys?.contains("q") ?: false
        }
        assertTrue("find safe qwant (url)") {
            qwantSafe?.url?.contains("safesearch=2") ?: false
        }

        val bingSafe = findBangUrlByKey("bi", safe=true)
        assertTrue("find safe bing (key)") {
            bingSafe?.keys?.contains("bi") ?: false
        }
        assertTrue("find safe bing (url)") {
            bingSafe?.url?.contains("adlt=strict") ?: false
        }
    }
}
