/**
 * Data Generation Script for romcal TypeScript package.
 *
 * This script generates JavaScript files from the JSON data files in data/definitions/
 * and data/resources/. The generated files enable tree-shaking so bundlers can
 * exclude unused calendars and locales.
 *
 * Usage: pnpm generate-data
 *
 * Output structure:
 *   dist/definitions/index.js     - Re-exports all calendars
 *   dist/definitions/{id}.js      - Individual calendar exports
 *   dist/resources/index.js       - Re-exports all locales
 *   dist/resources/{locale}.js    - Individual locale exports
 */

import { readFileSync, writeFileSync, mkdirSync, readdirSync, statSync } from 'node:fs'
import { join, dirname, relative } from 'node:path'
import { fileURLToPath } from 'node:url'
import { glob } from 'glob'

// Initialize WASM for merge_resource_files
import init, { merge_resource_files } from '../pkg/romcal_wasm.js'

const __dirname = dirname(fileURLToPath(import.meta.url))
const DATA_DIR = join(__dirname, '../../../data')
const DIST_DIR = join(__dirname, '../dist')

/**
 * Convert snake_case or kebab-case to camelCase.
 * Used for generating valid JavaScript identifiers.
 */
function toCamelCase(str: string): string {
  return str
    .replace(/[-_]([a-z])/g, (_, c) => c.toUpperCase())
    .replace(/^([A-Z])/, (_, c) => c.toLowerCase())
}

/**
 * Convert string to a valid JavaScript identifier.
 * Handles edge cases like strings starting with numbers.
 */
function toValidIdentifier(str: string): string {
  const camelCase = toCamelCase(str)
  // If starts with a number, prefix with underscore
  if (/^[0-9]/.test(camelCase)) {
    return `_${camelCase}`
  }
  return camelCase
}

/**
 * Generate JavaScript files for calendar definitions.
 *
 * Scans data/definitions/ recursively for JSON files and generates:
 * - Individual .js files with named exports
 * - An index.js that re-exports all calendars
 */
async function generateDefinitions(): Promise<void> {
  console.log('Generating calendar definitions...')

  const definitionsDir = join(DATA_DIR, 'definitions')
  const outputDir = join(DIST_DIR, 'definitions')
  mkdirSync(outputDir, { recursive: true })

  // Find all JSON files in definitions directory
  const pattern = join(definitionsDir, '**', '*.json').replace(/\\/g, '/')
  const files = await glob(pattern)

  const exports: string[] = []
  const generated: string[] = []

  for (const file of files) {
    try {
      const content = readFileSync(file, 'utf-8')
      const json = JSON.parse(content)
      const id = json.id as string

      if (!id) {
        console.warn(`  Skipping ${relative(DATA_DIR, file)}: no id field`)
        continue
      }

      const identifier = toValidIdentifier(id)

      // Generate the JS file
      const code = `// Generated from ${relative(DATA_DIR, file)}
// Calendar: ${id}
export const ${identifier} = ${JSON.stringify(json)};
`
      const outPath = join(outputDir, `${id}.js`)
      writeFileSync(outPath, code)

      // Also generate .d.ts for TypeScript
      const dtsCode = `import type { CalendarDefinition } from '../index.js';
export declare const ${identifier}: CalendarDefinition;
`
      writeFileSync(join(outputDir, `${id}.d.ts`), dtsCode)

      exports.push(`export { ${identifier} } from './${id}.js';`)
      generated.push(id)
    } catch (error) {
      console.error(`  Error processing ${file}:`, error)
    }
  }

  // Generate index.js
  const indexCode = `// Generated index - re-exports all calendar definitions
${exports.join('\n')}
`
  writeFileSync(join(outputDir, 'index.js'), indexCode)

  // Generate index.d.ts
  const indexDts = exports
    .map((e) => e.replace("from './", "from './").replace(".js'", ".js'"))
    .join('\n')
  writeFileSync(join(outputDir, 'index.d.ts'), indexDts)

  console.log(`  Generated ${generated.length} calendar definitions`)
}

/**
 * Generate JavaScript files for resources (locales).
 *
 * For each locale in data/resources/:
 * 1. Reads all JSON files (meta.json + entities.*.json)
 * 2. Uses merge_resource_files() to merge them
 * 3. Generates a .js file with the merged data
 */
async function generateResources(): Promise<void> {
  console.log('Generating resources...')

  const resourcesDir = join(DATA_DIR, 'resources')
  const outputDir = join(DIST_DIR, 'resources')
  mkdirSync(outputDir, { recursive: true })

  // Get all locale directories
  const locales = readdirSync(resourcesDir).filter((f) =>
    statSync(join(resourcesDir, f)).isDirectory(),
  )

  const exports: string[] = []
  const generated: string[] = []

  for (const locale of locales) {
    try {
      const localeDir = join(resourcesDir, locale)

      // Find all JSON files in the locale directory
      const pattern = join(localeDir, '*.json').replace(/\\/g, '/')
      const files = await glob(pattern)

      if (files.length === 0) {
        console.warn(`  Skipping ${locale}: no JSON files`)
        continue
      }

      // Read all files content
      const filesContent = files.map((f) => readFileSync(f, 'utf-8'))

      // Use WASM function to merge
      const mergedJson = merge_resource_files(locale, filesContent)
      const merged = JSON.parse(mergedJson)

      // Generate identifier (replace - with _ for valid JS identifier)
      const identifier = locale.replace(/-/g, '_')

      // Generate the JS file
      const code = `// Generated from data/resources/${locale}/
// Locale: ${locale}
export const ${identifier} = ${JSON.stringify(merged)};
`
      const outPath = join(outputDir, `${locale}.js`)
      writeFileSync(outPath, code)

      // Also generate .d.ts for TypeScript
      const dtsCode = `import type { Resources } from '../index.js';
export declare const ${identifier}: Resources;
`
      writeFileSync(join(outputDir, `${locale}.d.ts`), dtsCode)

      exports.push(`export { ${identifier} } from './${locale}.js';`)
      generated.push(locale)
    } catch (error) {
      console.error(`  Error processing locale ${locale}:`, error)
    }
  }

  // Generate index.js
  const indexCode = `// Generated index - re-exports all resources
${exports.join('\n')}
`
  writeFileSync(join(outputDir, 'index.js'), indexCode)

  // Generate index.d.ts
  const indexDts = exports.join('\n')
  writeFileSync(join(outputDir, 'index.d.ts'), indexDts)

  console.log(`  Generated ${generated.length} locale resources`)
}

/**
 * Main entry point.
 */
async function main(): Promise<void> {
  console.log('Romcal Data Generation')
  console.log('======================\n')

  // Initialize WASM (needed for merge_resource_files)
  console.log('Initializing WASM...')
  const wasmPath = join(__dirname, '../pkg/romcal_wasm_bg.wasm')
  const wasmBytes = readFileSync(wasmPath)
  await init({ module_or_path: wasmBytes })
  console.log('  WASM initialized\n')

  // Create dist directory
  mkdirSync(DIST_DIR, { recursive: true })

  // Generate definitions and resources
  await generateDefinitions()
  console.log()
  await generateResources()

  console.log('\nData generation complete!')
}

main().catch((error) => {
  console.error('Fatal error:', error)
  process.exit(1)
})
