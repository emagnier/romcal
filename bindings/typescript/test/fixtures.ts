import { CalendarDefinition, Resources } from '../src/index.js'
import { glob, readFile } from 'node:fs/promises'
import { dirname, basename, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
export const DATA_DIR = join(__dirname, '../../../data')

/**
 * Load all calendar definitions from the data folder
 */
export async function loadAllCalendarDefinitions(): Promise<CalendarDefinition[]> {
  const pattern = join(DATA_DIR, 'definitions/**/*.json')
  const definitions: CalendarDefinition[] = []

  for await (const file of glob(pattern)) {
    const content = await readFile(file, 'utf-8')
    definitions.push(JSON.parse(content))
  }

  return definitions
}

/**
 * Load all resources from the data folder
 * Each locale has meta.json + entities.*.json files that need to be merged
 */
export async function loadAllResources(): Promise<Resources[]> {
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

  // Merge files for each locale
  const resources: Resources[] = []
  for (const [locale, localeFiles] of filesByLocale) {
    let metadata: unknown = null
    const entities: Record<string, unknown> = {}

    for (const file of localeFiles) {
      const content = JSON.parse(await readFile(file, 'utf-8'))
      const fileName = basename(file)

      if (fileName === 'meta.json') {
        metadata = content.metadata
      } else if (fileName.startsWith('entities.')) {
        if (content.entities) {
          Object.assign(entities, content.entities)
        }
      }
    }

    resources.push({
      locale,
      metadata,
      entities: Object.keys(entities).length > 0 ? entities : null,
    })
  }

  return resources
}
