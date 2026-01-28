import { describe, it, expect, beforeAll } from 'vitest'
import { createRomcal, Romcal } from '../src/index.js'

// Import from generated files (simulating tree-shaking imports)
import { france } from '../dist/definitions/france.js'
import { generalRoman } from '../dist/definitions/general_roman.js'
import { europe } from '../dist/definitions/europe.js'
import { fr } from '../dist/resources/fr.js'
import { en } from '../dist/resources/en.js'

describe('generated definitions', () => {
  it('should have valid france calendar definition', () => {
    expect(france).toBeDefined()
    expect(france.id).toBe('france')
    expect(france.parent_calendar_ids).toContain('europe')
    expect(france.days_definitions).toBeDefined()
  })

  it('should have valid general_roman calendar definition', () => {
    expect(generalRoman).toBeDefined()
    expect(generalRoman.id).toBe('general_roman')
    expect(generalRoman.days_definitions).toBeDefined()
  })

  it('should have valid europe calendar definition', () => {
    expect(europe).toBeDefined()
    expect(europe.id).toBe('europe')
    // Europe is a top-level region, no parent except general_roman (handled implicitly)
    expect(europe.parent_calendar_ids).toEqual([])
  })
})

describe('generated resources', () => {
  it('should have valid fr resources', () => {
    expect(fr).toBeDefined()
    expect(fr.locale).toBe('fr')
    expect(fr.entities).toBeDefined()
  })

  it('should have valid en resources', () => {
    expect(en).toBeDefined()
    expect(en.locale).toBe('en')
    expect(en.entities).toBeDefined()
  })
})

describe('romcal with generated data', () => {
  let romcal: Romcal

  beforeAll(async () => {
    romcal = await createRomcal({
      calendar: 'france',
      locale: 'fr',
      calendarDefinitions: [france, europe, generalRoman],
      resources: [fr, en],
    })
  })

  it('should create romcal instance', () => {
    expect(romcal).toBeDefined()
    expect(romcal.calendar).toBe('france')
    expect(romcal.locale).toBe('fr')
  })

  it('should generate calendar with localized names', () => {
    const calendar = romcal.generateLiturgicalCalendar(2026)

    // Easter Sunday
    const easter = calendar['2026-04-05']
    expect(easter).toBeDefined()
    expect(easter[0].fullname).toBeDefined()
    expect(easter[0].rank_name).toBeDefined()
  })

  it('should include French-specific saints', () => {
    const calendar = romcal.generateLiturgicalCalendar(2026)

    // Saint Jean-Marie Vianney - August 4
    const vianney = calendar['2026-08-04']
    expect(vianney).toBeDefined()
    const saint = vianney.find((d) => d.id === 'john_mary_vianney_priest')
    expect(saint).toBeDefined()
  })
})
