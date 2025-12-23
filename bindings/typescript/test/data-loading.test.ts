import { describe, it, expect, beforeAll } from 'vitest'
import { createRomcal, CalendarDefinition, ResourcesDefinition, Romcal } from '../src/index.js'
import { loadAllCalendarDefinitions, loadAllResources } from './fixtures.js'

describe('data loading from /data folder', () => {
  let calendarDefinitions: CalendarDefinition[]
  let resources: ResourcesDefinition[]

  beforeAll(async () => {
    calendarDefinitions = await loadAllCalendarDefinitions()
    resources = await loadAllResources()
  })

  it('should load calendar definitions', async () => {
    expect(calendarDefinitions.length).toBeGreaterThan(0)
  })

  it('should load resource locales', async () => {
    expect(resources.length).toBeGreaterThan(0)
  })

  it('should have locale property in resources', async () => {
    for (const resource of resources) {
      expect(resource.locale).toBeDefined()
      expect(typeof resource.locale).toBe('string')
    }
  })
})

describe('French calendar with loaded data', () => {
  let romcal: Romcal
  let calendarDefinitions: CalendarDefinition[]
  let resources: ResourcesDefinition[]

  beforeAll(async () => {
    calendarDefinitions = await loadAllCalendarDefinitions()
    resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'france',
      locale: 'fr',
      calendarDefinitions,
      resources,
    })
  })

  it('should generate liturgical calendar with French locale', async () => {
    const calendar = await romcal.generateLiturgicalCalendar(2026)

    const easter = calendar['2026-04-05']
    expect(easter).toBeDefined()
    expect(easter[0].fullname).toBeDefined()
    expect(easter[0].rank_name).toBeDefined()
    expect(easter[0].season_name).toBeDefined()
  })

  it('should include French saints', async () => {
    const calendar = await romcal.generateLiturgicalCalendar(2026)

    // Saint Jean-Marie Vianney - August 4
    const vianney = calendar['2026-08-04']
    expect(vianney).toBeDefined()

    const saintDay = vianney.find((d) => d.id?.includes('vianney'))
    expect(saintDay).toBeDefined()
    expect(saintDay!.fullname).toBeDefined()
  })

  it('should generate mass calendar with French locale', async () => {
    const massCalendar = await romcal.generateMassCalendar(2026)

    // Default context is GREGORIAN, so Christmas 2026 is in the calendar
    const christmas = massCalendar['2026-12-25']
    expect(christmas).toBeDefined()
    expect(christmas.length).toBeGreaterThan(0)

    for (const mass of christmas) {
      expect(mass.mass_time_name).toBeDefined()
      expect(mass.fullname).toBeDefined()
    }
  })
})

describe('English calendar with loaded data', () => {
  let romcal: Romcal
  let calendarDefinitions: CalendarDefinition[]
  let resources: ResourcesDefinition[]

  beforeAll(async () => {
    calendarDefinitions = await loadAllCalendarDefinitions()
    resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      calendarDefinitions,
      resources,
    })
  })

  it('should generate liturgical calendar with English locale', async () => {
    const calendar = await romcal.generateLiturgicalCalendar(2026)

    const easter = calendar['2026-04-05']
    expect(easter).toBeDefined()
    expect(easter[0].fullname).toBeDefined()
    expect(easter[0].rank_name).toBeDefined()
    expect(easter[0].season_name).toBeDefined()
  })
})
