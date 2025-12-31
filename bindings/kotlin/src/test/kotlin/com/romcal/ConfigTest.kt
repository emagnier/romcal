package com.romcal

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ConfigTest {

    @Test
    fun `default configuration should have expected values`() {
        Romcal().use { romcal ->
            assertEquals("general_roman", romcal.calendar)
            assertEquals("en", romcal.locale)
            assertFalse(romcal.epiphanyOnSunday)
            assertFalse(romcal.ascensionOnSunday)
            assertTrue(romcal.corpusChristiOnSunday)
            assertEquals(EasterCalculationType.GREGORIAN, romcal.easterCalculationType)
            assertEquals(CalendarContext.GREGORIAN, romcal.context)
        }
    }

    @Test
    fun `should accept calendar and locale arguments`() {
        Romcal(calendar = "france", locale = "fr").use { romcal ->
            assertEquals("france", romcal.calendar)
            assertEquals("fr", romcal.locale)
        }
    }

    @Test
    fun `should accept partial configuration with defaults`() {
        Romcal(
            epiphanyOnSunday = true,
            ascensionOnSunday = true,
            corpusChristiOnSunday = false
        ).use { romcal ->
            assertTrue(romcal.epiphanyOnSunday)
            assertTrue(romcal.ascensionOnSunday)
            assertFalse(romcal.corpusChristiOnSunday)
            // Default values should still apply
            assertEquals("general_roman", romcal.calendar)
            assertEquals("en", romcal.locale)
        }
    }

    @Test
    fun `should accept easter calculation type configuration`() {
        Romcal(easterCalculationType = EasterCalculationType.JULIAN).use { romcal ->
            assertEquals(EasterCalculationType.JULIAN, romcal.easterCalculationType)
        }
    }

    @Test
    fun `should accept context configuration`() {
        Romcal(context = CalendarContext.LITURGICAL).use { romcal ->
            assertEquals(CalendarContext.LITURGICAL, romcal.context)
        }
    }
}
