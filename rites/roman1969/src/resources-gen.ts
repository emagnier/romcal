/**
 * Script to merge Martyrology and localization data
 *
 * This script:
 * - Extracts sources from comments in Martyrology and locales
 * - Merges Martyrology data with translated names
 * - Automatically deduplicates sources to avoid duplicates
 * - Generates JSON files organized by alphabetical letters
 * - Applies Prettier formatting to output files
 */

/* eslint-disable no-console */
import { writeFileSync, mkdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import prettier from 'prettier';

import { Martyrology } from './catalog/martyrology';
import { locales } from './locales';
import { locale as enLocale } from './locales/en';
import type { Locale } from './types/locale';

// Function to deduplicate sources
function deduplicateSources(sources: string[]): string[] {
  const seen = new Set<string>();
  const result: string[] = [];

  for (const source of sources) {
    const trimmedSource = source.trim();
    if (trimmedSource && !seen.has(trimmedSource)) {
      seen.add(trimmedSource);
      result.push(trimmedSource);
    }
  }

  return result;
}

// Function to split sources "mr_" separated by commas
function splitMrSources(sources: string[]): string[] {
  const result: string[] = [];

  for (const source of sources) {
    // If the source starts with "mr_" and contains commas
    if (source.startsWith('mr_') && source.includes(',')) {
      // Split by commas and clean each part
      const parts = source.split(',').map((part) => part.trim());
      result.push(...parts);
    } else {
      // Add source as is
      result.push(source);
    }
  }

  // Deduplicate sources after splitting
  return deduplicateSources(result);
}

// Function to format a JSON file with Prettier
async function formatJsonFile(filePath: string): Promise<void> {
  try {
    const content = readFileSync(filePath, 'utf8');
    const formatted = await prettier.format(content, {
      parser: 'json',
      printWidth: 80,
      tabWidth: 2,
      useTabs: false,
      semi: true,
      singleQuote: false,
      trailingComma: 'es5',
    });
    writeFileSync(filePath, formatted, 'utf8');
    console.log(`✨ Formatted file with Prettier: ${filePath}`);
  } catch (error) {
    console.warn(`⚠️  Error formatting with Prettier: ${filePath}`, error);
  }
}

// Function to extract sources from comments
function extractSourcesFromComments(martyrologyContent: string): Record<string, string[]> {
  const sources: Record<string, string[]> = {};

  // Split content into lines
  const lines = martyrologyContent.split('\n');

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Detect lines with "// src:"
    if (line.includes('// src:')) {
      // Search for the entry key (line following the comment block)
      let key = '';
      let j = i + 1;

      // Loop through the next lines to find the key
      while (j < lines.length) {
        const nextLine = lines[j].trim();

        // Ignore empty lines and comments
        if (nextLine && !nextLine.startsWith('//')) {
          // Check if it's an entry definition (key: {)
          if (nextLine.includes(':') && nextLine.includes('{')) {
            key = nextLine.split(':')[0].trim();
            break;
          }
        }
        j++;
      }

      if (key) {
        // Collect all sources in this block
        const sourceUrls: string[] = [];

        // Add the first line if it contains a source
        if (line.includes('// src:') && line.length > 8) {
          const srcPart = line.split('// src:')[1];
          if (srcPart?.trim()) {
            sourceUrls.push(srcPart.trim());
          }
        }

        // Loop through the next lines for sources with dashes
        j = i + 1;
        while (j < lines.length && lines[j].trim().startsWith('// -')) {
          const sourceLine = lines[j].trim();
          if (sourceLine.includes('// -')) {
            const sourcePart = sourceLine.split('// -')[1];
            if (sourcePart?.trim()) {
              sourceUrls.push(sourcePart.trim());
            }
          }
          j++;
        }

        if (sourceUrls.length > 0) {
          // Deduplicate sources before storing them
          sources[key] = deduplicateSources(sourceUrls);
        }
      }
    }
  }

  return sources;
}

// Function to convert TypeScript constants to readable values
function convertConstants(obj: unknown, sources?: Record<string, string[]>, entryKey?: string): unknown {
  if (obj === null || obj === undefined) {
    return obj;
  }

  if (typeof obj === 'object') {
    if (Array.isArray(obj)) {
      return obj.map((item) => convertConstants(item, sources, entryKey));
    }

    const converted: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj as Record<string, unknown>)) {
      converted[key] = convertConstants(value, sources, entryKey);
    }

    // Add sources only at the main entry level
    if (sources && entryKey && sources[entryKey]) {
      converted.sources = sources[entryKey];
    }

    return converted;
  }

  // Convert TypeScript constants to readable strings
  if (typeof obj === 'string' && obj.includes('CanonizationLevels.')) {
    return obj.replace('CanonizationLevels.', '');
  }

  if (typeof obj === 'string' && obj.includes('Title.')) {
    return obj.replace('Title.', '');
  }

  return obj;
}

// Function to merge Martyrology data with translated names
function mergeMartyrologyWithNames(
  _martyrologyData: Record<string, unknown>,
  _namesData: Record<string, string>
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
): Record<string, unknown> {
  const _mergedData: Record<string, unknown> = {};

  for (const [key, martyrologyEntry] of Object.entries(_martyrologyData)) {
    let mergedEntry = { ...(martyrologyEntry as Record<string, unknown>) };

    if (Object.prototype.hasOwnProperty.call(_namesData, key)) {
      mergedEntry = {
        fullname: _namesData[key],
        ...mergedEntry,
      };

      // Remove the 'name' property if it's identical to 'fullname'
      if (mergedEntry.name === mergedEntry.fullname) {
        const { name, ...entryWithoutName } = mergedEntry;
        mergedEntry = entryWithoutName;
      }
    }

    _mergedData[key] = mergedEntry;
  }

  for (const [key, value] of Object.entries(_namesData)) {
    if (!_martyrologyData[key]) {
      _mergedData[key] = {
        fullname: value,
      };
    }
  }

  return _mergedData;
}

// Function to sort an object by its keys alphabetically
function sortObjectByKeys(obj: Record<string, unknown>): Record<string, unknown> {
  const sortedKeys = Object.keys(obj).sort();
  const sortedObj: Record<string, unknown> = {};

  for (const key of sortedKeys) {
    sortedObj[key] = obj[key];
  }

  return sortedObj;
}

// Function to organize data by alphabetical letters
function organizeByAlphabet(data: Record<string, unknown>): Record<string, Record<string, unknown>> {
  const organized: Record<string, Record<string, unknown>> = {};

  for (const [key, value] of Object.entries(data)) {
    const firstLetter = key.charAt(0).toLowerCase();

    if (!organized[firstLetter]) {
      organized[firstLetter] = {};
    }

    organized[firstLetter][key] = value;
  }

  return organized;
}

// Function to extract sources from comments in locales
function extractLocaleSources(localeContent: string): Record<string, string[]> {
  const sources: Record<string, string[]> = {};

  // Split content into lines
  const lines = localeContent.split('\n');

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Search for lines that contain an entry definition
    if (line.includes(':') && line.includes('_') && !line.startsWith('//')) {
      // Extract the key
      const colonIndex = line.indexOf(':');
      if (colonIndex > 0) {
        const beforeColon = line.substring(0, colonIndex).trim();

        // Check if it's a key (contains underscores)
        if (beforeColon && beforeColon.length > 0 && beforeColon.includes('_')) {
          const key = beforeColon;

          // Check if there's a source comment on this line
          if (line.includes('//')) {
            const commentMatch = /\/\/\s*(.+)/.exec(line);
            if (commentMatch?.[1]) {
              const sourceComment = commentMatch[1].trim();
              // Remove "src: " if it's indicated
              const cleanSource = sourceComment.replace(/^src:\s*/, '');
              // Check if the key exists and add the source without duplicating
              if (sources[key]) {
                if (!sources[key].includes(cleanSource)) {
                  sources[key].push(cleanSource);
                }
              } else {
                sources[key] = [cleanSource];
              }
              console.log(`🔍 Source trouvée pour ${key}: ${cleanSource}`);
            }
          }

          // If the line ends with ":", check the next line for the comment
          if (line.trim().endsWith(':')) {
            const nextLine = lines[i + 1];
            if (nextLine?.includes('//')) {
              const commentMatch = /\/\/\s*(.+)/.exec(nextLine);
              if (commentMatch?.[1]) {
                const sourceComment = commentMatch[1].trim();
                // Remove "src: " if it's indicated
                const cleanSource = sourceComment.replace(/^src:\s*/, '');
                // Check if the key exists and add the source without duplicating
                if (sources[key]) {
                  if (!sources[key].includes(cleanSource)) {
                    sources[key].push(cleanSource);
                  }
                } else {
                  sources[key] = [cleanSource];
                }
                console.log(`🔍 Source found for ${key} (next line): ${cleanSource}`);
              }
            }
          }
        }
      }
    }
  }

  return sources;
}

// Function to convert locale name to folder format
function getLocaleFolderName(localeCode: string): string {
  // Convert to lowercase and replace uppercase letters with dashes
  // Example: EnGb -> en-gb, PtBr -> pt-br, Cs -> cs
  return localeCode
    .replace(/([A-Z])/g, (match, p1, offset) => {
      // Add a dash before the uppercase letter, except at the beginning
      return offset === 0 ? p1.toLowerCase() : `-${p1.toLowerCase()}`;
    })
    .toLowerCase();
}

// Processing English locale with martyrology merge
console.log('🚀 Start processing locales...');

// Read the content of the martyrology.ts file to extract sources
const martyrologyFilePath = join(__dirname, 'catalog', 'martyrology.ts');
const martyrologyContent = readFileSync(martyrologyFilePath, 'utf8');

// Read the content of the en.ts file to extract sources
const enLocaleFilePath = join(__dirname, 'locales', 'en.ts');
const enLocaleContent = readFileSync(enLocaleFilePath, 'utf8');

// Extract sources from comments in martyrology and en locale
const martyrologySources = extractSourcesFromComments(martyrologyContent);
const enLocaleSources = extractLocaleSources(enLocaleContent);

// Merge sources (the sources of the en locale have priority)
// and deduplicate sources for each key
const sources: Record<string, string[]> = {};
for (const [key, martyrologySourceList] of Object.entries(martyrologySources)) {
  sources[key] = martyrologySourceList;
}
for (const [key, localeSourceList] of Object.entries(enLocaleSources)) {
  if (sources[key]) {
    // Merge and deduplicate sources
    const allSources = [...sources[key], ...localeSourceList];
    sources[key] = deduplicateSources(allSources);
  } else {
    sources[key] = localeSourceList;
  }
}

// Convert martyrology to JSON object
const martyrologyData: Record<string, unknown> = {};
for (const [key, value] of Object.entries(Martyrology.catalog)) {
  martyrologyData[key] = convertConstants(value, sources, key);
}

// Extract translated names from the en locale
const namesData = enLocale.names as Record<string, string>;

// Merge data
const mergedData = mergeMartyrologyWithNames(martyrologyData, namesData);

// Apply extracted sources from locales to merged entries
for (const [key, value] of Object.entries(mergedData)) {
  if (sources[key] && typeof value === 'object' && value !== null) {
    const sourceUrls = splitMrSources(sources[key]);
    const todoSources: string[] = [];
    const normalSources: string[] = [];

    // Split sources containing "Todo" from normal sources
    for (const source of sourceUrls) {
      if (source.toLowerCase().includes('todo')) {
        todoSources.push(source);
      } else {
        normalSources.push(source);
      }
    }

    // Deduplicate normal sources and Todo
    const uniqueNormalSources = deduplicateSources(normalSources);
    const uniqueTodoSources = deduplicateSources(todoSources);

    // Add normal sources to the "sources" property
    if (uniqueNormalSources.length > 0) {
      (value as Record<string, unknown>).sources = uniqueNormalSources;
    }

    // Add Todo sources to the "_todo" property
    if (uniqueTodoSources.length > 0) {
      (value as Record<string, unknown>)._todo = uniqueTodoSources;
    }
  }
}

// Sort data by alphabetical keys
const sortedData = sortObjectByKeys(mergedData);

// Organize data by alphabetical letters
const organizedData = organizeByAlphabet(sortedData);

// Create the output directory for the en locale
const outputPath = join(__dirname, 'resources', 'en');

// Create the directory if it doesn't exist
try {
  mkdirSync(outputPath, { recursive: true });
} catch {
  // The directory already exists or permission error
}

// Write JSON files by letter for the en locale
async function writeEnglishLocaleFiles(): Promise<void> {
  try {
    let totalEntries = 0;
    const letters = Object.keys(organizedData).sort();

    for (const letter of letters) {
      const letterData = organizedData[letter];
      const letterFilePath = join(outputPath, `${letter}.json`);

      const finalData = {
        locale: 'en',
        letter,
        items: letterData,
      };

      writeFileSync(letterFilePath, JSON.stringify(finalData, null, 2), 'utf8');

      // Format the file with Prettier
      await formatJsonFile(letterFilePath);

      totalEntries += Object.keys(letterData).length;

      console.log(`📝 File ${letter}.json created with ${Object.keys(letterData).length} entries`);
    }

    console.log(`✅ English locale with martyrology merged successfully in: ${outputPath}`);
    console.log(`📊 Martyrology entries: ${Object.keys(martyrologyData).length}`);
    console.log(`📊 Translated names: ${Object.keys(namesData).length}`);
    console.log(`📊 Merged entries: ${Object.keys(mergedData).length}`);
    console.log(`📊 Total entries in all files: ${totalEntries}`);
    console.log(`📊 Created files: ${letters.length} (${letters.join(', ')})`);
    console.log('📊 Data sorted by alphabetical keys and organized by letters');

    // Count entries with fullname
    const entriesWithFullname = Object.values(mergedData).filter(
      (entry) => typeof entry === 'object' && entry !== null && 'fullname' in entry
    ).length;
    console.log(`📊 Entries with translated name: ${entriesWithFullname}`);

    // Create meta.json file for the en locale
    const metaFilePath = join(outputPath, 'meta.json');
    const metaContent = {
      locale: 'en',
      metadata: {
        seasons: enLocale.seasons || {},
        periods: enLocale.periods || {},
        ranks: enLocale.ranks || {},
        cycles: enLocale.cycles || {},
        weekdays: enLocale.weekdays || {},
        months: enLocale.months || {},
        colors: enLocale.colors || {},
        ordinals: enLocale.ordinals || {},
      },
    };

    writeFileSync(metaFilePath, JSON.stringify(metaContent, null, 2), 'utf8');
    await formatJsonFile(metaFilePath);
    console.log('📝 File meta.json created for locale en');
  } catch (error) {
    console.error('❌ Error writing files:', error);
  }
}

// Call the asynchronous function to write the en locale files
writeEnglishLocaleFiles();

// Function to process a locale with its sources
async function processLocaleWithSources(
  localeCode: string,
  localeData: Locale,
  localeSources: Record<string, string[]>
): Promise<void> {
  console.log(`🔄 Processing locale: ${localeCode}`);

  // Extract translated names
  const localeNamesData = (localeData.names as Record<string, string>) || {};

  // Transform data into structure {fullname: value, sources?: string[], _todo?: string[]}
  const localeTransformedData: Record<string, { fullname: string; sources?: string[]; _todo?: string[] }> = {};
  for (const [key, value] of Object.entries(localeNamesData)) {
    const entry: { fullname: string; sources?: string[]; _todo?: string[] } = { fullname: value };

    // Add sources if available
    if (localeSources[key]) {
      const sourceUrls = splitMrSources(localeSources[key]);
      const todoSources: string[] = [];
      const normalSources: string[] = [];

      // Split sources containing "Todo" from normal sources
      for (const source of sourceUrls) {
        if (source.toLowerCase().includes('todo')) {
          todoSources.push(source);
        } else {
          normalSources.push(source);
        }
      }

      // Deduplicate normal sources and Todo
      const uniqueNormalSources = deduplicateSources(normalSources);
      const uniqueTodoSources = deduplicateSources(todoSources);

      // Add normal sources to the "sources" property
      if (uniqueNormalSources.length > 0) {
        entry.sources = uniqueNormalSources;
      }

      // Add Todo sources to the "_todo" property
      if (uniqueTodoSources.length > 0) {
        entry._todo = uniqueTodoSources;
      }
    }

    localeTransformedData[key] = entry;
  }

  // Sort by alphabetical keys
  const localeSortedData = sortObjectByKeys(localeTransformedData);

  // Organize by alphabetical letters
  const localeOrganizedData = organizeByAlphabet(localeSortedData);

  // Create the output directory for this locale
  const localeFolderName = getLocaleFolderName(localeCode);
  const localeOutputDir = join(__dirname, 'resources', localeFolderName);
  mkdirSync(localeOutputDir, { recursive: true });

  // Write files organized by letters
  let localeTotalEntries = 0;
  const localeCreatedFiles: string[] = [];

  for (const [letter, letterData] of Object.entries(localeOrganizedData)) {
    const localeOutputPath = join(localeOutputDir, `${letter}.json`);
    const localeOutputContent = {
      locale: localeFolderName,
      letter,
      items: letterData,
    };

    writeFileSync(localeOutputPath, JSON.stringify(localeOutputContent, null, 2), 'utf8');

    // Format the file with Prettier
    await formatJsonFile(localeOutputPath);

    console.log(`📝 File ${letter}.json created with ${Object.keys(letterData).length} entries`);

    localeTotalEntries += Object.keys(letterData).length;
    localeCreatedFiles.push(letter);
  }

  // Create meta.json file with all metadata for this locale
  const metaFilePath = join(localeOutputDir, 'meta.json');
  const metaContent = {
    locale: localeFolderName,
    metadata: {
      seasons: localeData.seasons || {},
      periods: localeData.periods || {},
      ranks: localeData.ranks || {},
      cycles: localeData.cycles || {},
      weekdays: localeData.weekdays || {},
      months: localeData.months || {},
      colors: localeData.colors || {},
      ordinals: localeData.ordinals || {},
    },
  };

  writeFileSync(metaFilePath, JSON.stringify(metaContent, null, 2), 'utf8');
  await formatJsonFile(metaFilePath);
  console.log(`📝 File meta.json created for locale ${localeCode}`);

  console.log(`✅ Locale ${localeCode} exported successfully in: ${localeOutputDir}`);
  console.log(`📊 Total entries: ${localeTotalEntries}`);
  console.log(`📊 Created files: ${localeCreatedFiles.length + 1} (${localeCreatedFiles.join(', ')}, meta.json)`);
  console.log('');
}

// Process all other locales
async function processAllLocales(): Promise<void> {
  const localeEntries = Object.entries(locales);
  for (const [localeCode, localeData] of localeEntries) {
    // Ignore the en locale because it has already been processed with martyrology
    if (localeCode === 'En') continue;

    // Read the content of the locale file to extract sources
    const localeFilePath = join(__dirname, 'locales', `${getLocaleFolderName(localeCode)}.ts`);
    let localeContent = '';
    try {
      localeContent = readFileSync(localeFilePath, 'utf8');
    } catch {
      // If the file doesn't exist, continue without sources
      console.log(`⚠️  Locale file not found: ${localeFilePath}`);
    }

    // Extract sources from the locale
    const localeSources = extractLocaleSources(localeContent);

    // Process the locale with its sources
    await processLocaleWithSources(localeCode, localeData, localeSources);
  }
}

// Call the asynchronous function to process all locales
processAllLocales();

console.log('\n All locales processed successfully!');
