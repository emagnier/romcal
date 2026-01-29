import {
  CalendarDefinition,
  Resources,
  mergeResourceFiles,
  mergeCalendarDefinitions,
} from '../src/index.js'
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
  const files: object[] = []

  for await (const file of glob(pattern)) {
    const content = await readFile(file, 'utf-8')
    files.push(JSON.parse(content))
  }

  return mergeCalendarDefinitions(files)
}

/**
 * Load all resources from the data folder
 * Each locale has meta.json + martyrology.*.json files that need to be merged
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
