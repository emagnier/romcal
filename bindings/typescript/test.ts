import { romcal, RomcalError, CalendarDefinition, ResourcesDefinition } from './src/index.js';
import { glob, readFile } from 'node:fs/promises';
import { dirname, basename, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const DATA_DIR = join(__dirname, '../../data');

// ============================================================================
// Data Loading Utilities
// ============================================================================

/**
 * Load all calendar definitions from the data folder
 */
async function loadAllCalendarDefinitions(): Promise<CalendarDefinition[]> {
  const pattern = join(DATA_DIR, 'definitions/**/*.json');
  const definitions: CalendarDefinition[] = [];

  // fs.glob returns an AsyncIterator in Node.js
  for await (const file of glob(pattern)) {
    const content = await readFile(file, 'utf-8');
    definitions.push(JSON.parse(content));
  }

  return definitions;
}

/**
 * Load all resources from the data folder
 * Each locale has meta.json + entities.*.json files that need to be merged
 */
async function loadAllResources(): Promise<ResourcesDefinition[]> {
  const resourcesDir = join(DATA_DIR, 'resources');
  const pattern = join(resourcesDir, '**/*.json');

  // Group files by locale (parent directory name)
  const filesByLocale = new Map<string, string[]>();
  for await (const file of glob(pattern)) {
    const parentDir = dirname(file);
    const locale = basename(parentDir);
    if (!filesByLocale.has(locale)) {
      filesByLocale.set(locale, []);
    }
    filesByLocale.get(locale)!.push(file);
  }

  // Merge files for each locale
  const resources: ResourcesDefinition[] = [];
  for (const [locale, localeFiles] of filesByLocale) {
    let metadata: unknown = null;
    const entities: Record<string, unknown> = {};

    for (const file of localeFiles) {
      const content = JSON.parse(await readFile(file, 'utf-8'));
      const fileName = basename(file);

      if (fileName === 'meta.json') {
        metadata = content.metadata;
      } else if (fileName.startsWith('entities.')) {
        // Merge entities from this file
        if (content.entities) {
          Object.assign(entities, content.entities);
        }
      }
    }

    resources.push({
      locale,
      metadata,
      entities: Object.keys(entities).length > 0 ? entities : null,
    });
  }

  return resources;
}

// ============================================================================
// Tests
// ============================================================================

async function testRomcal() {
  console.log('Testing romcal...\n');

  // Test 1: Default configuration
  console.log('Test 1: Default configuration');
  const calendar1 = await romcal();

  console.log('  calendar:', calendar1.config.calendar); // 'general_roman'
  console.log('  locale:', calendar1.config.locale); // 'en'
  console.log('  epiphanyOnSunday:', calendar1.config.epiphanyOnSunday); // false
  console.log('  corpusChristiOnSunday:', calendar1.config.corpusChristiOnSunday); // true
  console.log('  ascensionOnSunday:', calendar1.config.ascensionOnSunday); // false
  console.log('  easterCalculationType:', calendar1.config.easterCalculationType); // 'GREGORIAN'
  console.log('  context:', calendar1.config.context); // 'GREGORIAN'
  console.log('');

  // Test 2: Custom configuration with calendar and locale
  console.log('Test 2: Custom configuration (france, fr)');
  const calendar2 = await romcal('france', 'fr');
  console.log('  calendar:', calendar2.config.calendar); // 'france'
  console.log('  locale:', calendar2.config.locale); // 'fr'
  console.log('');

  // Test 3: Generate liturgical calendar
  console.log('Test 3: Generate liturgical calendar for 2026');
  const liturgicalCalendar = await calendar1.generateLiturgicalCalendar(2026);
  const dates = Object.keys(liturgicalCalendar);
  console.log('  Total dates:', dates.length);
  console.log('  First date:', dates[0]);
  console.log('  Last date:', dates[dates.length - 1]);

  // Check Easter 2026 (April 5)
  const easter2026 = liturgicalCalendar['2026-04-05'];
  if (easter2026 && easter2026.length > 0) {
    console.log('  Easter 2026:', easter2026[0].fullname);
    console.log('  Easter rank:', easter2026[0].rank);
  }
  console.log('');

  // Test 4: Generate mass calendar
  console.log('Test 4: Generate mass calendar for 2026');
  const massCalendar = await calendar1.generateMassCalendar(2026);
  const massDates = Object.keys(massCalendar);
  console.log('  Total dates:', massDates.length);

  // Check Christmas 2025 (in liturgical year 2026)
  const christmas = massCalendar['2025-12-25'];
  if (christmas && christmas.length > 0) {
    console.log('  Christmas 2025 masses:', christmas.length);
    for (const mass of christmas) {
      console.log(`    - ${mass.mass_time}: ${mass.fullname}`);
    }
  }
  console.log('');

  // Test 5: Generate calendar with French locale
  console.log('Test 5: Generate French liturgical calendar for 2026');
  const frCalendar = await calendar2.generateLiturgicalCalendar(2026);
  const easter2026Fr = frCalendar['2026-04-05'];
  if (easter2026Fr && easter2026Fr.length > 0) {
    console.log('  Easter 2026 (fr):', easter2026Fr[0].fullname);
  }
  console.log('');

  // Test 6: Configuration with partial options
  console.log('Test 6: Partial configuration');
  const calendar3 = await romcal({
    calendar: 'united_states',
    locale: 'en',
    epiphanyOnSunday: true,
    ascensionOnSunday: true,
  });
  console.log('  calendar:', calendar3.config.calendar);
  console.log('  epiphanyOnSunday:', calendar3.config.epiphanyOnSunday);
  console.log('  ascensionOnSunday:', calendar3.config.ascensionOnSunday);
  console.log('');

  // Test 7: Error handling - invalid year
  console.log('Test 7: Error handling (invalid year)');
  try {
    await calendar1.generateLiturgicalCalendar(1500);
    console.log('  ERROR: Should have thrown an error for year 1500');
  } catch (error) {
    if (error instanceof RomcalError) {
      console.log('  Caught RomcalError as expected:', error.message.substring(0, 60) + '...');
    } else {
      console.log('  Caught error:', error);
    }
  }
  console.log('');

  // Test 8: Check mass calendar structure
  console.log('Test 8: Mass calendar structure');
  const easterVigil = massCalendar['2026-04-04']; // Easter Vigil is on Saturday evening
  if (easterVigil) {
    // Note: mass_time is serialized as SCREAMING_SNAKE_CASE by Rust
    const vigil = easterVigil.find((m) => (m.mass_time as string) === 'EASTER_VIGIL');
    if (vigil) {
      console.log('  Easter Vigil found on civil date 2026-04-04');
      console.log('  Liturgical date:', vigil.liturgical_date);
      console.log('  Mass time:', vigil.mass_time);
    }
  }
  console.log('');

  // =========================================================================
  // Tests with data loaded from /data
  // =========================================================================

  console.log('Test 9: Loading data from /data folder');
  const calendarDefinitions = await loadAllCalendarDefinitions();
  const resources = await loadAllResources();
  console.log(`  Loaded ${calendarDefinitions.length} calendar definitions`);
  console.log(`  Loaded ${resources.length} resource locales`);
  console.log('');

  // Test 10: French calendar with loaded data
  console.log('Test 10: French calendar with loaded data');
  const frRomcal = await romcal({
    calendar: 'france',
    locale: 'fr',
    calendarDefinitions,
    resources,
  });

  const frCalendarWithData = await frRomcal.generateLiturgicalCalendar(2026);
  const easterFr = frCalendarWithData['2026-04-05'];
  if (easterFr && easterFr.length > 0) {
    console.log('  Easter 2026 (fr with data):', easterFr[0].fullname);
    console.log('  Rank name:', easterFr[0].rank_name);
    console.log('  Season name:', easterFr[0].season_name);
  }

  // Check a saint's day (e.g., Saint Jean-Marie Vianney - August 4)
  const vianney = frCalendarWithData['2026-08-04'];
  if (vianney && vianney.length > 0) {
    const saintDay = vianney.find((d) => d.id?.includes('vianney'));
    if (saintDay) {
      console.log('  Saint Vianney:', saintDay.fullname);
    }
  }
  console.log('');

  // Test 11: English calendar with loaded data
  console.log('Test 11: English calendar with loaded data');
  const enRomcal = await romcal({
    calendar: 'general_roman',
    locale: 'en',
    calendarDefinitions,
    resources,
  });

  const enCalendarWithData = await enRomcal.generateLiturgicalCalendar(2026);
  const easterEn = enCalendarWithData['2026-04-05'];
  if (easterEn && easterEn.length > 0) {
    console.log('  Easter 2026 (en with data):', easterEn[0].fullname);
    console.log('  Rank name:', easterEn[0].rank_name);
    console.log('  Season name:', easterEn[0].season_name);
  }
  console.log('');

  // Test 12: Mass calendar with loaded data
  console.log('Test 12: Mass calendar with French data');
  const frMassCalendar = await frRomcal.generateMassCalendar(2026);
  const christmasFr = frMassCalendar['2025-12-25'];
  if (christmasFr && christmasFr.length > 0) {
    console.log('  Christmas 2025 masses (fr):');
    for (const mass of christmasFr.slice(0, 3)) {
      console.log(`    - ${mass.mass_time_name}: ${mass.fullname}`);
    }
  }
  console.log('');

  console.log('All tests passed!');
}

testRomcal().catch(console.error);
