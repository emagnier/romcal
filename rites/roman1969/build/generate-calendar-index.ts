import fs from 'node:fs';
import path from 'node:path';

// Main calendar categories
const MAIN_CATEGORIES = ['communities', 'countries', 'general_roman', 'regions'] as const;

// Cache for calendar IDs to avoid multiple file reads
const calendarIdCache = new Map<string, string>();

interface CalendarConfig {
  id: string;
  parentCalendarIds: string[];
  particularConfig?: Record<string, unknown>;
  inputs: Record<string, unknown>;
}

/**
 * Detect automatically all calendar directories
 */
function detectCalendarDirectories(calendarsPath: string): string[] {
  const directories: string[] = [];

  // Check main categories
  for (const category of MAIN_CATEGORIES) {
    const categoryPath = path.join(calendarsPath, category);

    try {
      if (!fs.existsSync(categoryPath)) {
        console.warn(`⚠️ Category directory not found: ${categoryPath}`);
        continue;
      }

      const entries = fs.readdirSync(categoryPath, { withFileTypes: true });

      const subdirs = entries
        .filter((dirent) => dirent.isDirectory())
        .map((dirent) => path.join(category, dirent.name));
      directories.push(...subdirs);

      // For categories that contain files directly (like general_roman, regions)
      // add the category itself as a directory
      const hasFiles = entries.some((dirent) => dirent.isFile() && dirent.name.endsWith('.json'));

      if (hasFiles) {
        directories.push(category);
      }
    } catch (error) {
      console.error(`❌ Error processing category ${category}:`, error);
    }
  }

  return directories;
}

/**
 * Detect automatically all JSON files in a calendar directory
 */
function detectJsonFiles(calendarPath: string): string[] {
  try {
    return fs
      .readdirSync(calendarPath)
      .filter((file) => file.endsWith('.json'))
      .map((file) => file.replace('.json', ''));
  } catch (error) {
    console.warn(`⚠️ Error reading directory ${calendarPath}:`, error);
    return [];
  }
}

/**
 * Convert snake_case to PascalCase
 */
function formatConstantName(calendarId: string): string {
  return calendarId
    .split('__')
    .map((part) =>
      part
        .split('_')
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
        .join('')
    )
    .join('');
}

/**
 * Get calendar ID from JSON file with caching
 */
function getCalendarId(jsonFilePath: string): string {
  // Check cache first
  if (calendarIdCache.has(jsonFilePath)) {
    const cachedId = calendarIdCache.get(jsonFilePath);
    if (cachedId) {
      return cachedId;
    }
  }

  try {
    const content = fs.readFileSync(jsonFilePath, 'utf8');
    const calendar: CalendarConfig = JSON.parse(content);
    const calendarId = calendar.id;

    // Cache the result
    calendarIdCache.set(jsonFilePath, calendarId);
    return calendarId;
  } catch (error) {
    console.warn(`⚠️ Error reading calendar ID from ${jsonFilePath}:`, error);
    // Fallback: use filename
    const fallbackId = path.basename(jsonFilePath, '.json');
    calendarIdCache.set(jsonFilePath, fallbackId);
    return fallbackId;
  }
}

/**
 * Validate file path exists and is accessible
 */
function validateFilePath(filePath: string): boolean {
  try {
    return fs.existsSync(filePath) && fs.statSync(filePath).isFile();
  } catch {
    return false;
  }
}

/**
 * Get calendar metadata (ID and constant name) for a JSON file
 */
function getCalendarMetadata(jsonFilePath: string): { calendarId: string; constantName: string } {
  const calendarId = getCalendarId(jsonFilePath);
  const constantName = formatConstantName(calendarId);
  return { calendarId, constantName };
}

/**
 * Check if a file exists and has identical content
 */
function isFileIdentical(filePath: string, newContent: string): boolean {
  if (!fs.existsSync(filePath)) {
    return false;
  }

  try {
    const existingContent = fs.readFileSync(filePath, 'utf8');
    return existingContent === newContent;
  } catch (error) {
    console.warn(`⚠️ Error reading file ${filePath}:`, error);
    return false;
  }
}

/**
 * Generate _filename.ts file for a specific JSON calendar file
 */
function generateCalendarJsonTs(jsonFilePath: string): void {
  const calendarId = getCalendarId(jsonFilePath);
  const constantName = formatConstantName(calendarId);
  const jsonFileName = path.basename(jsonFilePath, '.json');
  const jsonDir = path.dirname(jsonFilePath);

  const content = `import ${constantName} from './${jsonFileName}.json';

export { ${constantName} };
`;

  const outputPath = path.join(jsonDir, `_${jsonFileName}.ts`);

  // Check if file exists and has identical content
  if (isFileIdentical(outputPath, content)) {
    return;
  }

  try {
    fs.writeFileSync(outputPath, content);
    console.log(`✅ Generated _${jsonFileName}.ts (exported as ${constantName})`);
  } catch (error) {
    console.error(`❌ Error writing file ${outputPath}:`, error);
  }
}

/**
 * Generate _index.ts file for a specific calendar directory
 */
function generateCalendarDirectoryIndex(calendarPath: string): void {
  const jsonFiles = detectJsonFiles(calendarPath);

  if (jsonFiles.length === 0) {
    return;
  }

  // Only export the index.json file (main calendar for this directory)
  const indexFilePath = path.join(calendarPath, 'index.json');

  if (!validateFilePath(indexFilePath)) {
    // Skip warning for regions directory as it doesn't have an index.json by design
    if (!calendarPath.endsWith('regions')) {
      console.warn(`⚠️ No index.json found in directory: ${calendarPath}`);
    }
    return;
  }

  const { constantName } = getCalendarMetadata(indexFilePath);

  const content = `import ${constantName} from './index.json';

export { ${constantName} };
`;

  const indexPath = path.join(calendarPath, '_index.ts');

  // Check if file exists and has identical content
  if (isFileIdentical(indexPath, content)) {
    return;
  }

  try {
    fs.writeFileSync(indexPath, content);
    console.log(`✅ Generated _index.ts for ${path.basename(calendarPath)} (exported ${constantName})`);
  } catch (error) {
    console.error(`❌ Error writing file ${indexPath}:`, error);
  }
}

/**
 * Generate the main _index.ts file at the root of calendars
 */
function generateMainIndex(calendarsPath: string, directoryInfo: { path: string; jsonFiles: string[] }[]): void {
  const allJsonFiles: string[] = [];

  // Collect all JSON files from directory info
  for (const { path: calendarPath, jsonFiles } of directoryInfo) {
    const relativePath = path.relative(calendarsPath, calendarPath);
    allJsonFiles.push(...jsonFiles.map((file) => path.join(relativePath, file)));
  }

  // Generate direct exports with proper path handling
  const directExports: string[] = [];

  for (const relativePath of allJsonFiles) {
    const jsonFilePath = path.join(calendarsPath, `${relativePath}.json`);

    if (!validateFilePath(jsonFilePath)) {
      console.warn(`⚠️ JSON file not found or invalid: ${jsonFilePath}`);
      continue;
    }

    const { constantName } = getCalendarMetadata(jsonFilePath);

    // Generate import path for individual calendar file
    const fileName = path.basename(relativePath);
    const dirPath = path.dirname(relativePath);

    // If it's an index.json file, export from the directory's _index.ts
    // Otherwise, export from the individual _filename.ts file
    let importPath: string;
    if (fileName === 'index') {
      importPath = dirPath === '.' ? '_index' : `${dirPath}/_index`;
    } else {
      importPath = dirPath === '.' ? `_${fileName}` : `${dirPath}/_${fileName}`;
    }

    directExports.push(`export { ${constantName} } from './${importPath}';`);
  }

  const content = `${directExports.join('\n')}
`;

  const mainIndexPath = path.join(calendarsPath, '_index.ts');

  // Check if file exists and has identical content
  if (isFileIdentical(mainIndexPath, content)) {
    return;
  }

  try {
    fs.writeFileSync(mainIndexPath, content);
    console.log(`✅ Generated main _index.ts with ${allJsonFiles.length} calendar exports`);
  } catch (error) {
    console.error(`❌ Error writing file ${mainIndexPath}:`, error);
  }
}

function main(): void {
  // Get the absolute path to the src directory from the build directory
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const buildDir = path.dirname(new URL(import.meta.url).pathname);
  const projectRoot = path.resolve(buildDir, '..');
  const calendarsPath = path.join(projectRoot, 'src', 'calendars');

  if (!fs.existsSync(calendarsPath)) {
    console.error(`❌ Calendars directory not found: ${calendarsPath}`);
    process.exit(1);
  }

  try {
    // Detect automatically all calendar directories
    const calendarDirectories = detectCalendarDirectories(calendarsPath);

    // Count total calendars and collect directory info
    let totalCalendars = 0;
    const directoryInfo: { path: string; jsonFiles: string[] }[] = [];

    for (const calendarDir of calendarDirectories) {
      const calendarPath = path.join(calendarsPath, calendarDir);
      const jsonFiles = detectJsonFiles(calendarPath);
      totalCalendars += jsonFiles.length;
      directoryInfo.push({ path: calendarPath, jsonFiles });
    }

    console.log(`🔍 Found ${calendarDirectories.length} calendar directories with ${totalCalendars} total calendars`);

    // Generate _filename.ts files for each JSON file
    for (const { path: calendarPath, jsonFiles } of directoryInfo) {
      for (const jsonFile of jsonFiles) {
        const jsonFilePath = path.join(calendarPath, `${jsonFile}.json`);
        generateCalendarJsonTs(jsonFilePath);
      }

      // Generate _index.ts for this directory
      generateCalendarDirectoryIndex(calendarPath);
    }

    // Generate the main _index.ts file at the root of calendars
    generateMainIndex(calendarsPath, directoryInfo);

    console.log('✨ All calendar index files are up to date!');
  } catch (error) {
    console.error('❌ Error during calendar index generation:', error);
    console.error('Stack trace:', error instanceof Error ? error.stack : 'No stack trace available');
    process.exit(1);
  }
}

// Execute the script
main();
