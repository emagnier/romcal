import { describe, it, expect } from 'vitest'
import { createRomcal, RomcalError } from '../src/index.js'

describe('Romcal configuration', () => {
  it('should use default configuration', async () => {
    const romcal = await createRomcal()

    expect(romcal.calendar).toBe('general_roman')
    expect(romcal.locale).toBe('en')
    expect(romcal.epiphanyOnSunday).toBe(false)
    expect(romcal.corpusChristiOnSunday).toBe(true)
    expect(romcal.ascensionOnSunday).toBe(false)
    expect(romcal.easterCalculationType).toBe('gregorian')
    expect(romcal.context).toBe('gregorian')
  })

  it('should accept calendar and locale as arguments', async () => {
    const romcal = await createRomcal('france', 'fr')

    expect(romcal.calendar).toBe('france')
    expect(romcal.locale).toBe('fr')
  })

  it('should accept partial configuration object', async () => {
    const romcal = await createRomcal({
      calendar: 'united_states',
      locale: 'en',
      epiphanyOnSunday: true,
      ascensionOnSunday: true,
    })

    expect(romcal.calendar).toBe('united_states')
    expect(romcal.locale).toBe('en')
    expect(romcal.epiphanyOnSunday).toBe(true)
    expect(romcal.ascensionOnSunday).toBe(true)
    // Default values preserved
    expect(romcal.corpusChristiOnSunday).toBe(true)
  })

  it('should throw RomcalError for invalid easter calculation type', async () => {
    await expect(createRomcal({ easterCalculationType: 'invalid' as never })).rejects.toThrow(
      RomcalError,
    )
    await expect(createRomcal({ easterCalculationType: 'invalid' as never })).rejects.toThrow(
      "Invalid easter_calculation_type: 'invalid'",
    )
  })

  it('should throw RomcalError for invalid context', async () => {
    await expect(createRomcal({ context: 'invalid' as never })).rejects.toThrow(RomcalError)
    await expect(createRomcal({ context: 'invalid' as never })).rejects.toThrow(
      "Invalid context: 'invalid'",
    )
  })
})
