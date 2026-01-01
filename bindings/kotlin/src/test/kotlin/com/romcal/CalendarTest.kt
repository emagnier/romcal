package com.romcal

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class CalendarTest {
  @Test
  fun `should generate gregorian year calendar`() {
    Romcal().use { romcal ->
      val calendar = romcal.liturgicalCalendar(2025)

      // Should have entries for the whole year
      assertTrue(calendar.isNotEmpty())

      // January 1st should exist
      val jan1 = calendar["2025-01-01"]
      assertNotNull(jan1)
      assertTrue(jan1.isNotEmpty())

      // Check that it has expected properties
      val firstDay = jan1[0]
      assertTrue(firstDay.containsKey("id"))
      assertTrue(firstDay.containsKey("rank"))
      assertTrue(firstDay.containsKey("season"))
    }
  }

  @Test
  fun `should generate liturgical year calendar`() {
    Romcal(context = CalendarContext.LITURGICAL).use { romcal ->
      val calendar = romcal.liturgicalCalendar(2025)

      // Should have entries
      assertTrue(calendar.isNotEmpty())

      // First Sunday of Advent 2024 should be included (start of liturgical year 2025)
      val firstSundayOfAdvent = calendar["2024-12-01"]
      assertNotNull(firstSundayOfAdvent, "First Sunday of Advent should be included")
    }
  }

  @Test
  fun `should get Easter date for 2025`() {
    Romcal().use { romcal ->
      val calendar = romcal.liturgicalCalendar(2025)

      // Find Easter by looking for the date with Easter Sunday
      val easterDate =
        calendar.entries.find { (_, days) ->
          days.any { day ->
            val id = day["id"] as? String
            id?.contains("easter_sunday") == true
          }
        }?.key

      assertEquals("2025-04-20", easterDate)
    }
  }

  @Test
  fun `should get Christmas date`() {
    Romcal().use { romcal ->
      val christmas = romcal.getDate("christmas", 2025)
      assertEquals("2025-12-25", christmas)
    }
  }

  @Test
  fun `should have correct season for Advent dates`() {
    Romcal(context = CalendarContext.LITURGICAL).use { romcal ->
      val calendar = romcal.liturgicalCalendar(2025)

      // First Sunday of Advent 2024
      val adventDay = calendar["2024-12-01"]
      assertNotNull(adventDay)
      assertTrue(adventDay.isNotEmpty())
      assertEquals("ADVENT", adventDay[0]["season"])
    }
  }

  @Test
  fun `Christmas should be a solemnity`() {
    Romcal().use { romcal ->
      val calendar = romcal.liturgicalCalendar(2025)

      val christmas = calendar["2025-12-25"]
      assertNotNull(christmas)
      assertTrue(christmas.isNotEmpty())
      assertEquals("SOLEMNITY", christmas[0]["rank"])
    }
  }

  @Test
  fun `should generate mass calendar`() {
    Romcal().use { romcal ->
      val massCalendar = romcal.massCalendar(2025)

      // Should have entries
      assertTrue(massCalendar.isNotEmpty())

      // Check a specific date
      val christmas = massCalendar["2025-12-25"]
      assertNotNull(christmas)
      assertTrue(christmas.isNotEmpty())

      // Check that it has expected properties (mass_time for MassContext)
      val firstMass = christmas[0]
      assertTrue(
        firstMass.containsKey("mass_time"),
        "Expected 'mass_time' key in mass context, got keys: ${firstMass.keys}",
      )
    }
  }

  @Test
  fun `version should be available`() {
    val version = Romcal.version
    assertTrue(version.isNotEmpty())
    assertTrue(version.startsWith("4."))
  }
}
