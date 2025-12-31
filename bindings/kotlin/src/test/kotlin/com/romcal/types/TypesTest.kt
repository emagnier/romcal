package com.romcal.types

import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class TypesTest {

    private val json = Json { ignoreUnknownKeys = true }

    @Test
    fun `Rank enum should have correct values`() {
        assertEquals("SOLEMNITY", Rank.Solemnity.string)
        assertEquals("SUNDAY", Rank.Sunday.string)
        assertEquals("FEAST", Rank.Feast.string)
        assertEquals("MEMORIAL", Rank.Memorial.string)
        assertEquals("OPTIONAL_MEMORIAL", Rank.OptionalMemorial.string)
        assertEquals("WEEKDAY", Rank.Weekday.string)
    }

    @Test
    fun `Color enum should have correct values`() {
        assertEquals("RED", Color.Red.string)
        assertEquals("WHITE", Color.White.string)
        assertEquals("GREEN", Color.Green.string)
        assertEquals("PURPLE", Color.Purple.string)
        assertEquals("ROSE", Color.Rose.string)
        assertEquals("GOLD", Color.Gold.string)
        assertEquals("BLACK", Color.Black.string)
    }

    @Test
    fun `Season enum should have correct values`() {
        assertEquals("ADVENT", Season.Advent.string)
        assertEquals("CHRISTMAS_TIME", Season.ChristmasTime.string)
        assertEquals("LENT", Season.Lent.string)
        assertEquals("PASCHAL_TRIDUUM", Season.PaschalTriduum.string)
        assertEquals("EASTER_TIME", Season.EasterTime.string)
        assertEquals("ORDINARY_TIME", Season.OrdinaryTime.string)
    }

    @Test
    fun `CalendarType enum should have correct values`() {
        assertEquals("GENERAL_ROMAN", CalendarType.GeneralRoman.string)
        assertEquals("COUNTRY", CalendarType.Country.string)
        assertEquals("DIOCESE", CalendarType.Diocese.string)
    }

    @Test
    fun `ColorInfo should deserialize correctly`() {
        val jsonString = """{"key": "RED", "name": "Red"}"""
        val colorInfo = json.decodeFromString<ColorInfo>(jsonString)
        assertEquals(Color.Red, colorInfo.key)
        assertEquals("Red", colorInfo.name)
    }

    @Test
    fun `CalendarMetadata should deserialize correctly`() {
        val jsonString = """{"type": "COUNTRY", "jurisdiction": "CIVIL"}"""
        val metadata = json.decodeFromString<CalendarMetadata>(jsonString)
        assertEquals(CalendarType.Country, metadata.type)
        assertEquals(CalendarJurisdiction.Civil, metadata.jurisdiction)
    }

    @Test
    fun `MassInfo should deserialize correctly`() {
        // Note: Typeshare generates SerialName with PascalCase format
        val jsonString = """{"type": "DayMass", "name": "Day Mass"}"""
        val massInfo = json.decodeFromString<MassInfo>(jsonString)
        assertEquals(MassTime.DayMass, massInfo.type)
        assertEquals("Day Mass", massInfo.name)
    }

    @Test
    fun `Precedence enum should have correct values`() {
        assertEquals("TRIDUUM_1", Precedence.Triduum_1.string)
        assertEquals("GENERAL_SOLEMNITY_3", Precedence.GeneralSolemnity_3.string)
        assertEquals("WEEKDAY_13", Precedence.Weekday_13.string)
    }

    @Test
    fun `SundayCycle enum should have correct values`() {
        assertEquals("YEAR_A", SundayCycle.YearA.string)
        assertEquals("YEAR_B", SundayCycle.YearB.string)
        assertEquals("YEAR_C", SundayCycle.YearC.string)
    }

    @Test
    fun `WeekdayCycle enum should have correct values`() {
        assertEquals("YEAR_1", WeekdayCycle.Year_1.string)
        assertEquals("YEAR_2", WeekdayCycle.Year_2.string)
    }

    @Test
    fun `PsalterWeekCycle enum should have correct values`() {
        assertEquals("WEEK_1", PsalterWeekCycle.Week_1.string)
        assertEquals("WEEK_2", PsalterWeekCycle.Week_2.string)
        assertEquals("WEEK_3", PsalterWeekCycle.Week_3.string)
        assertEquals("WEEK_4", PsalterWeekCycle.Week_4.string)
    }

    @Test
    fun `BTreeMap typealias should work as Map`() {
        val map: BTreeMap<String, Int> = mapOf("one" to 1, "two" to 2)
        assertEquals(2, map.size)
        assertEquals(1, map["one"])
    }
}
