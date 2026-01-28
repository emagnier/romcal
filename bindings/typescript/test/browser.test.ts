import { describe, it, expect, beforeAll } from 'vitest'
import type { createRomcal } from '../src/index.js'

declare global {
  interface Window {
    Romcal: {
      createRomcal: typeof createRomcal
    }
  }
}

describe('Browser ESM bundle', () => {
  it('should load romcal and create an instance', async () => {
    // Import the built ESM module (Vite will serve it)
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()

    expect(romcal.calendar).toBe('general_roman')
    expect(romcal.locale).toBe('en')
  })

  it('should generate a liturgical calendar', async () => {
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()
    const calendar = romcal.generateLiturgicalCalendar(2026)

    expect(calendar['2026-04-05']).toBeDefined() // Easter
    expect(calendar['2026-04-05'][0].id).toBe('easter_sunday')
  })

  it('should get a specific date', async () => {
    const { createRomcal } = await import('../dist/romcal.js')

    const romcal = await createRomcal()
    const easter = romcal.getDate('easter_sunday', 2026)

    expect(easter).toBe('2026-04-05')
  })
})

describe('Browser UMD bundle (script tag)', () => {
  beforeAll(async () => {
    // Inject the UMD script into the page (like a real browser would)
    await new Promise<void>((resolve, reject) => {
      const script = document.createElement('script')
      script.src = '/dist/romcal.umd.js'
      script.onload = () => resolve()
      script.onerror = () => reject(new Error('Failed to load UMD script'))
      document.head.appendChild(script)
    })
  })

  it('should expose Romcal global variable', () => {
    expect(window.Romcal).toBeDefined()
  })

  it('should have createRomcal function', () => {
    expect(typeof window.Romcal.createRomcal).toBe('function')
  })

  it('should create an instance via global', async () => {
    const romcal = await window.Romcal.createRomcal()

    expect(romcal.calendar).toBe('general_roman')
    expect(romcal.locale).toBe('en')
  })

  it('should generate a liturgical calendar via global', async () => {
    const romcal = await window.Romcal.createRomcal()
    const calendar = romcal.generateLiturgicalCalendar(2026)

    expect(calendar['2026-04-05']).toBeDefined()
    expect(calendar['2026-04-05'][0].id).toBe('easter_sunday')
  })
})
