import { describe, it, expect, beforeAll } from 'vitest'
import {
  createRomcal,
  RomcalError,
  Romcal,
  LiturgicalCalendar,
  MassCalendar,
} from '../src/index.js'
import { loadAllCalendarDefinitions, loadAllResources } from './fixtures.js'

describe('Gregorian year calendar (default)', () => {
  let romcal: Romcal
  let calendar: LiturgicalCalendar

  beforeAll(async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      calendarDefinitions,
      resources,
      // context: 'GREGORIAN' is the default
    })
    calendar = await romcal.generateLiturgicalCalendar(2026)
  })

  it('should generate a full Gregorian year', async () => {
    const dates = Object.keys(calendar)
    expect(dates.length).toBeGreaterThanOrEqual(365)
  })

  it('should start on January 1 and end on December 31', async () => {
    const dates = Object.keys(calendar).sort()
    expect(dates[0]).toBe('2026-01-01')
    expect(dates[dates.length - 1]).toBe('2026-12-31')
  })

  it('should include Easter 2026 on April 5', async () => {
    const easter = calendar['2026-04-05']

    expect(easter).toBeDefined()
    expect(easter.length).toBeGreaterThan(0)
    // Easter has the highest precedence (TRIDUUM_1)
    expect(easter[0].precedence).toBe('TRIDUUM_1')
    expect(easter[0].is_holy_day_of_obligation).toBe(true)
  })

  it('should have correct Easter season', async () => {
    const easter = calendar['2026-04-05']

    expect(easter[0].fullname).toBe('Easter Sunday of the Resurrection of the Lord')
    expect(easter[0].season).toBe('EASTER_TIME')
  })

  it('should include Christmas 2026 on December 25', async () => {
    const christmas = calendar['2026-12-25']

    expect(christmas).toBeDefined()
    expect(christmas[0].rank).toBe('SOLEMNITY')
  })

  it('should have no masses defined for Holy Saturday (April 4)', async () => {
    const holySaturday = calendar['2026-04-04']

    expect(holySaturday).toBeDefined()
    expect(holySaturday[0].fullname).toBe('Holy Saturday')
    // Holy Saturday has no masses during the day (only Easter Vigil in the evening)
    expect(holySaturday[0].masses).toEqual([])
  })
})

describe('Liturgical year calendar', () => {
  let romcal: Romcal
  let calendar: LiturgicalCalendar

  beforeAll(async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      context: 'LITURGICAL',
      calendarDefinitions,
      resources,
    })
    calendar = await romcal.generateLiturgicalCalendar(2026)
  })

  it('should generate a full liturgical year', async () => {
    const dates = Object.keys(calendar)
    expect(dates.length).toBeGreaterThan(350)
  })

  it('should start in late November 2025 (Advent)', async () => {
    const dates = Object.keys(calendar).sort()
    // Liturgical year 2026 starts on first Sunday of Advent 2025
    expect(dates[0]).toBe('2025-11-30')
  })

  it('should end in late November 2026 (Saturday after Christ the King)', async () => {
    const dates = Object.keys(calendar).sort()
    expect(dates[dates.length - 1]).toBe('2026-11-28')
  })

  it('should include Christmas 2025', async () => {
    const christmas = calendar['2025-12-25']

    expect(christmas).toBeDefined()
    expect(christmas[0].rank).toBe('SOLEMNITY')
  })
})

describe('Mass calendar (Gregorian year)', () => {
  let romcal: Romcal
  let massCalendar: MassCalendar

  beforeAll(async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      calendarDefinitions,
      resources,
    })
    massCalendar = await romcal.generateMassCalendar(2026)
  })

  it('should generate mass calendar', async () => {
    const dates = Object.keys(massCalendar)
    expect(dates.length).toBeGreaterThanOrEqual(365)
  })

  it('should include December 24 masses (morning + Christmas vigil)', async () => {
    const dec24 = massCalendar['2026-12-24']
    const massTimes = dec24.map((t) => t.mass_time)

    expect(dec24).toBeDefined()
    expect(dec24.length).toBe(2)
    // Morning mass (Advent weekday) + Previous evening mass (Christmas vigil)
    expect(massTimes).toStrictEqual(['MORNING_MASS', 'PREVIOUS_EVENING_MASS'])
  })

  it('should include multiple Christmas 2026 masses', async () => {
    const christmas = massCalendar['2026-12-25']
    const massTimes = christmas.map((t) => t.mass_time)

    expect(christmas).toBeDefined()
    expect(christmas.length).toBe(3)
    expect(massTimes).toStrictEqual(['NIGHT_MASS', 'MASS_AT_DAWN', 'DAY_MASS'])
  })

  it('should have correct mass time names', async () => {
    const christmas = massCalendar['2026-12-25']

    for (const mass of christmas) {
      expect(mass.mass_time).toBeDefined()
      expect(mass.fullname).toContain('The Nativity of the Lord')
    }
  })

  it('should place Easter Vigil on Saturday evening (April 4)', async () => {
    const easterVigilDay = massCalendar['2026-04-04']

    expect(easterVigilDay).toBeDefined()

    const vigil = easterVigilDay.find((m) => (m.mass_time as string) === 'EASTER_VIGIL')
    expect(vigil).toBeDefined()
    expect(vigil!.liturgical_date).toBe('2026-04-05')
  })

  it('should have a mass entry for Holy Saturday in mass-centric view', async () => {
    // Holy Saturday has no masses in the liturgical day definition,
    // but the mass-centric calendar includes the Easter Vigil on this civil date
    const holySaturday = massCalendar['2026-04-04']

    expect(holySaturday).toBeDefined()
    expect(holySaturday.length).toBe(1)
    expect(holySaturday[0].mass_time).toBe('EASTER_VIGIL')
  })
})

describe('Mass calendar (Liturgical year)', () => {
  let romcal: Romcal
  let massCalendar: MassCalendar

  beforeAll(async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      context: 'LITURGICAL',
      calendarDefinitions,
      resources,
    })
    massCalendar = await romcal.generateMassCalendar(2026)
  })

  it('should include Christmas 2025 masses', async () => {
    const christmas = massCalendar['2025-12-25']

    expect(christmas).toBeDefined()
    expect(christmas.length).toBeGreaterThan(1)

    for (const mass of christmas) {
      expect(mass.mass_time).toBeDefined()
      expect(mass.fullname).toContain('The Nativity of the Lord')
    }
  })
})

describe('French calendar', () => {
  it('should generate French locale calendar', async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    const romcal = await createRomcal({
      calendar: 'france',
      locale: 'fr',
      calendarDefinitions,
      resources,
    })
    const calendar = await romcal.generateLiturgicalCalendar(2026)

    const easter = calendar['2026-04-05']
    expect(easter).toBeDefined()
    expect(easter[0].fullname).toBe('Dimanche de Pâques - La résurrection du Seigneur')
  })
})

describe('error handling', () => {
  it('should throw RomcalError for invalid year', async () => {
    const romcal = await createRomcal()

    await expect(romcal.generateLiturgicalCalendar(1500)).rejects.toThrow(RomcalError)
  })

  it('should include error message for invalid year', async () => {
    const romcal = await createRomcal()

    try {
      await romcal.generateLiturgicalCalendar(1500)
      expect.fail('Should have thrown an error')
    } catch (error) {
      expect(error).toBeInstanceOf(RomcalError)
      expect((error as RomcalError).message).toContain('1500')
    }
  })

  it('should accept year 1583 (first valid Gregorian year)', async () => {
    const romcal = await createRomcal()
    const calendar = await romcal.generateLiturgicalCalendar(1583)
    expect(Object.keys(calendar).length).toBeGreaterThan(0)
  })

  it('should reject year 1582 (before Gregorian calendar)', async () => {
    const romcal = await createRomcal()
    await expect(romcal.generateLiturgicalCalendar(1582)).rejects.toThrow(RomcalError)
  })

  it('should throw RomcalError with validation message for year < 1583', async () => {
    const romcal = await createRomcal()

    try {
      await romcal.generateLiturgicalCalendar(1500)
      expect.fail('Should have thrown an error')
    } catch (error) {
      expect(error).toBeInstanceOf(RomcalError)
      // Year validation errors are thrown directly (no cause)
      // WASM errors would have a cause when wrapped
      expect((error as RomcalError).message).toContain('1583')
    }
  })
})
