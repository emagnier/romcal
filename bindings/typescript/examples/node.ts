/// <reference types="node" />

/**
 * Romcal - Node.js Example
 *
 * This example demonstrates:
 * - Loading calendar definitions and resources from the data folder
 * - Creating a Romcal instance with custom configuration
 * - Generating liturgical and mass calendars
 *
 * Run with: npx tsx examples/node.ts
 */

import {
  createRomcal,
  CalendarDefinition,
  Resources,
  mergeResourceFiles,
  mergeCalendarDefinitions,
} from '../src/index.js'
import { glob, readFile } from 'node:fs/promises'
import { dirname, basename, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const DATA_DIR = join(__dirname, '../../../data')

// ============================================================================
// Data Loading Utilities
// ============================================================================

/**
 * Load all calendar definitions from the data folder
 */
async function loadAllCalendarDefinitions(): Promise<CalendarDefinition[]> {
  const pattern = join(DATA_DIR, 'definitions/**/*.json')
  const files: object[] = []

  for await (const file of glob(pattern)) {
    const content = await readFile(file, 'utf-8')
    files.push(JSON.parse(content))
  }

  return mergeCalendarDefinitions(files)
}

/**
 * Load all resources from the data folder
 * Each locale has meta.json + entities.*.json files that need to be merged
 */
async function loadAllResources(): Promise<Resources[]> {
  const resourcesDir = join(DATA_DIR, 'resources')
  const pattern = join(resourcesDir, '**/*.json')

  // Group files by locale (parent directory name)
  const filesByLocale = new Map<string, string[]>()
  for await (const file of glob(pattern)) {
    const parentDir = dirname(file)
    const locale = basename(parentDir)
    if (!filesByLocale.has(locale)) {
      filesByLocale.set(locale, [])
    }
    filesByLocale.get(locale)!.push(file)
  }

  // Merge files for each locale using the helper
  const resources: Resources[] = []
  for (const [locale, localeFiles] of filesByLocale) {
    const filesContent = await Promise.all(
      localeFiles.map((f) => readFile(f, 'utf-8').then((content) => JSON.parse(content))),
    )
    resources.push(await mergeResourceFiles(locale, filesContent))
  }

  return resources
}

// ============================================================================
// Main Example
// ============================================================================

async function main() {
  console.log('=== Romcal Node.js Example ===\n')

  // Load data from the data folder
  console.log('Loading calendar definitions and resources...')
  const calendarDefinitions = await loadAllCalendarDefinitions()
  const resources = await loadAllResources()
  console.log(`  Loaded ${calendarDefinitions.length} calendar definitions`)
  console.log(`  Loaded ${resources.length} resource locales\n`)

  // Create a French calendar instance
  console.log('Creating French calendar instance...')
  const romcal = await createRomcal({
    calendar: 'france',
    locale: 'fr',
    calendarDefinitions,
    resources,
  })
  console.log(`  Calendar: ${romcal.config.calendar}`)
  console.log(`  Locale: ${romcal.config.locale}\n`)

  // Generate liturgical calendar for 2026
  console.log('Generating liturgical calendar for 2026...')
  const liturgicalCalendar = await romcal.generateLiturgicalCalendar(2026)
  const dates = Object.keys(liturgicalCalendar)
  console.log(`  Total dates: ${dates.length}`)
  console.log(`  First date: ${dates[0]}`)
  console.log(`  Last date: ${dates[dates.length - 1]}\n`)

  // Display some notable dates
  console.log('Notable celebrations:')

  // Easter 2026
  const easter = liturgicalCalendar['2026-04-05']
  if (easter?.[0]) {
    console.log(`  Easter (2026-04-05): ${easter[0].fullname}`)
    console.log(`    Season: ${easter[0].season_name}`)
    console.log(`    Rank: ${easter[0].rank_name}`)
  }

  // Pentecost (50 days after Easter)
  const pentecost = liturgicalCalendar['2026-05-24']
  if (pentecost?.[0]) {
    console.log(`  Pentecost (2026-05-24): ${pentecost[0].fullname}`)
  }

  // Assumption of Mary
  const assumption = liturgicalCalendar['2026-08-15']
  if (assumption?.[0]) {
    console.log(`  Assumption (2026-08-15): ${assumption[0].fullname}`)
  }

  // All Saints
  const allSaints = liturgicalCalendar['2026-11-01']
  if (allSaints?.[0]) {
    console.log(`  All Saints (2026-11-01): ${allSaints[0].fullname}`)
  }

  // Christmas 2026
  const christmas = liturgicalCalendar['2026-12-25']
  if (christmas?.[0]) {
    console.log(`  Christmas (2026-12-25): ${christmas[0].fullname}`)
  }

  console.log()

  // Generate mass calendar
  console.log('Generating mass calendar for 2026...')
  const massCalendar = await romcal.generateMassCalendar(2026)

  // Show Christmas masses
  const christmasMasses = massCalendar['2025-12-25']
  if (christmasMasses) {
    console.log(`  Christmas masses (${christmasMasses.length} total):`)
    for (const mass of christmasMasses.slice(0, 4)) {
      console.log(`    - ${mass.mass_time_name}: ${mass.fullname}`)
    }
  }

  // Show Easter Vigil (on April 4th evening, for April 5th liturgical date)
  const easterVigilDay = massCalendar['2026-04-04']
  if (easterVigilDay) {
    const vigil = easterVigilDay.find((m) => (m.mass_time as string) === 'EASTER_VIGIL')
    if (vigil) {
      console.log(`  Easter Vigil (civil: 2026-04-04, liturgical: ${vigil.liturgical_date}):`)
      console.log(`    - ${vigil.mass_time_name}: ${vigil.fullname}`)
    }
  }

  console.log('\n=== Done ===')
}

main().catch(console.error)
