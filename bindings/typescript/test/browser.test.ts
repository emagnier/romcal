import { describe, it, expect } from 'vitest'

describe('Browser ESM bundle', () => {
  it('should load romcal and create an instance', async () => {
    // Import the built ESM module (Vite will serve it)
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()

    expect(romcal.config).toBeDefined()
    expect(romcal.config.calendar).toBe('general_roman')
  })

  it('should generate a liturgical calendar', async () => {
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()
    const calendar = await romcal.generateLiturgicalCalendar(2026)

    expect(calendar['2026-04-05']).toBeDefined() // Easter
    expect(calendar['2026-04-05'][0].id).toBe('easter_sunday')
  })

  it('should get a specific date', async () => {
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()
    const easter = await romcal.getDate('easter_sunday', 2026)

    expect(easter).toBe('2026-04-05')
  })
})
