import { describe, it, expect } from 'vitest'
import { createRomcal, RomcalError } from '../src/index.js'

describe('Romcal configuration', () => {
  it('should use default configuration', async () => {
    const romcal = await createRomcal()

    expect(romcal.config.calendar).toBe('general_roman')
    expect(romcal.config.locale).toBe('en')
    expect(romcal.config.epiphanyOnSunday).toBe(false)
    expect(romcal.config.corpusChristiOnSunday).toBe(true)
    expect(romcal.config.ascensionOnSunday).toBe(false)
    expect(romcal.config.easterCalculationType).toBe('GREGORIAN')
    expect(romcal.config.context).toBe('GREGORIAN')
  })

  it('should accept calendar and locale as arguments', async () => {
    const romcal = await createRomcal('france', 'fr')

    expect(romcal.config.calendar).toBe('france')
    expect(romcal.config.locale).toBe('fr')
  })

  it('should accept partial configuration object', async () => {
    const romcal = await createRomcal({
      calendar: 'united_states',
      locale: 'en',
      epiphanyOnSunday: true,
      ascensionOnSunday: true,
    })

    expect(romcal.config.calendar).toBe('united_states')
    expect(romcal.config.locale).toBe('en')
    expect(romcal.config.epiphanyOnSunday).toBe(true)
    expect(romcal.config.ascensionOnSunday).toBe(true)
    // Default values preserved
    expect(romcal.config.corpusChristiOnSunday).toBe(true)
  })

  it('should throw RomcalError for invalid easter calculation type', async () => {
    await expect(createRomcal({ easterCalculationType: 'INVALID' as never })).rejects.toThrow(
      RomcalError,
    )
    await expect(createRomcal({ easterCalculationType: 'INVALID' as never })).rejects.toThrow(
      "Invalid easter_calculation_type: 'INVALID'",
    )
  })

  it('should throw RomcalError for invalid context', async () => {
    await expect(createRomcal({ context: 'INVALID' as never })).rejects.toThrow(RomcalError)
    await expect(createRomcal({ context: 'INVALID' as never })).rejects.toThrow(
      "Invalid context: 'INVALID'",
    )
  })
})
