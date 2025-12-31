package com.romcal

import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class DataLoadingTest {

    @Test
    fun `should load calendar definitions from data folder`() {
        val definitionsJson = Fixtures.loadAllCalendarDefinitionsJson()
        assertTrue(definitionsJson.isNotEmpty())
        assertTrue(definitionsJson.startsWith("["))
    }

    @Test
    fun `should load resources from data folder`() {
        val resourcesJson = Fixtures.loadAllResourcesJson()
        assertTrue(resourcesJson.isNotEmpty())
        assertTrue(resourcesJson.startsWith("["))
    }

    @Test
    fun `should load French resources`() {
        val frenchResourcesJson = Fixtures.loadResourcesForLocale("fr")
        assertTrue(frenchResourcesJson.isNotEmpty())
        assertTrue(frenchResourcesJson.contains("\"locale\":\"fr\""))
    }

    @Test
    fun `should generate calendar with loaded data`() {
        val definitionsJson = Fixtures.loadAllCalendarDefinitionsJson()
        val resourcesJson = Fixtures.loadAllResourcesJson()

        Romcal(
            calendar = "france",
            locale = "fr",
            calendarDefinitionsJson = definitionsJson,
            resourcesJson = resourcesJson
        ).use { romcal ->
            val calendar = romcal.liturgicalCalendar(2025)
            assertTrue(calendar.isNotEmpty())

            // Check Christmas has French name
            val christmas = calendar["2025-12-25"]
            assertNotNull(christmas)
            assertTrue(christmas.isNotEmpty())
            // The fullname should be present
            val fullname = christmas[0]["fullname"] as? String
            assertNotNull(fullname)
            assertTrue(fullname.isNotEmpty())
        }
    }

    @Test
    fun `should generate mass calendar with French locale`() {
        val definitionsJson = Fixtures.loadAllCalendarDefinitionsJson()
        val resourcesJson = Fixtures.loadAllResourcesJson()

        Romcal(
            calendar = "france",
            locale = "fr",
            calendarDefinitionsJson = definitionsJson,
            resourcesJson = resourcesJson
        ).use { romcal ->
            val massCalendar = romcal.massCalendar(2025)
            assertTrue(massCalendar.isNotEmpty())
        }
    }
}
