import fs from 'node:fs';
import path from 'node:path';

import { type CalendarDef, type CalendarDefInputs, type ParticularConfig, RomcalConfig } from '@src/rite-roman1969';

interface CalendarConfig {
  name: string;
  path: string;
  sourceFile: string;
  outputFile: string;
  allExports: string[];
  module: Record<string, unknown>;
}

/**
 * Detect automatically all calendar directories
 */
function detectCalendarDirectories(calendarsPath: string): string[] {
  const directories: string[] = [];

  // Check main categories
  const mainCategories = ['countries', 'regions', 'general_roman', 'communities'];

  for (const category of mainCategories) {
    const categoryPath = path.join(calendarsPath, category);
    if (fs.existsSync(categoryPath)) {
      const subdirs = fs
        .readdirSync(categoryPath, { withFileTypes: true })
        .filter((dirent) => dirent.isDirectory())
        .map((dirent) => path.join(category, dirent.name));
      directories.push(...subdirs);

      // For categories that contain files directly (like general-roman, regions)
      // add the category itself as a directory
      const hasFiles = fs
        .readdirSync(categoryPath, { withFileTypes: true })
        .some((dirent) => dirent.isFile() && dirent.name.endsWith('.ts'));

      if (hasFiles) {
        directories.push(category);
      }
    }
  }

  return directories;
}

/**
 * Detect automatically all TypeScript files in a calendar directory
 * Ignore files that start with underscore
 */
function detectTsFiles(calendarPath: string): string[] {
  return fs
    .readdirSync(calendarPath)
    .filter((file) => file.endsWith('.ts') && !file.startsWith('_'))
    .map((file) => file.replace('.ts', ''));
}

type CalendarDefJson = {
  id: string;
  metadata?: {
    type?: string;
    jurisdiction?: string;
    established?: string;
    patron?: string;
  };
  parentCalendarIds: string[];
  particularConfig?: ParticularConfig;
  inputs: CalendarDefInputs;
};

/**
 * Generate metadata based on the source filename and directory context
 */
function generateMetadata(sourceFileName: string, directoryPath: string): { type?: string; jurisdiction?: string } {
  const metadata: { type?: string; jurisdiction?: string } = {
    jurisdiction: 'ECCLESIASTICAL',
  };

  // Extract type from filename
  if (sourceFileName.includes('archdiocese-of-')) {
    metadata.type = 'ARCHDIOCESE';
  } else if (sourceFileName.includes('diocese-of-')) {
    metadata.type = 'DIOCESE';
  } else if (directoryPath.includes('/countries/')) {
    // Files in countries directory that are not dioceses/archdioceses are countries
    metadata.type = 'COUNTRY';
    metadata.jurisdiction = 'CIVIL';
  } else if (directoryPath.endsWith('/regions')) {
    // Files directly in regions directory are regions
    metadata.type = 'REGION';
    metadata.jurisdiction = 'CIVIL';
  } else if (directoryPath.includes('/communities/')) {
    // Files in communities directory are religious communities
    metadata.type = 'COMMUNITY';
    metadata.jurisdiction = 'ECCLESIASTICAL';
  } else if (directoryPath.endsWith('/general_roman')) {
    // Files in general_roman directory are general roman calendar
    metadata.type = 'GENERAL_ROMAN';
    metadata.jurisdiction = 'ECCLESIASTICAL';
  }

  return metadata;
}

function serializeCalendarDef(cal: CalendarDef, sourceFileName?: string, directoryPath?: string): CalendarDefJson {
  const metadata = sourceFileName && directoryPath ? generateMetadata(sourceFileName, directoryPath) : undefined;

  return JSON.parse(
    JSON.stringify({
      id: cal.calendarName,
      metadata,
      parentCalendarIds: cal.ParentCalendars?.map((parent) => parent.name) ?? [],
      particularConfig: cal.particularConfig,
      inputs: cal.inputs,
    })
  ) as CalendarDefJson;
}

async function main(): Promise<void> {
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

    // console.log(`🔍 Found ${calendarDirectories.length} calendar directories:`, calendarDirectories);

    const calData: CalendarConfig[] = [];

    // Generate JSON for each calendar
    for (const calendarDir of calendarDirectories) {
      const calendarPath = path.join(calendarsPath, calendarDir);
      const tsFiles = detectTsFiles(calendarPath);

      for (const tsFile of tsFiles) {
        const sourceFile = path.join(calendarPath, `${tsFile}.ts`);
        const outputFile = path.join(calendarPath, `${tsFile}.json`);

        // Use absolute path for dynamic import
        const absoluteSourceFile = path.resolve(sourceFile);
        const sourceModule = await import(`file://${absoluteSourceFile}`);
        const classNames = Object.keys(sourceModule);

        calData.push({
          name: '',
          allExports: classNames,
          path: calendarPath,
          sourceFile,
          outputFile,
          module: sourceModule,
        });
      }
    }

    const usedNames = new Set<string>();
    const calConfigs = calData
      .sort((a, b) => a.allExports.length - b.allExports.length)
      .map((c) => {
        const name: string =
          c.allExports.length === 1 ? c.allExports[0] : c.allExports.filter((e) => !usedNames.has(e))[0] || '';

        if (usedNames.has(name)) throw new Error(`Name ${name} is already used`);
        if (name === '') throw new Error(`No name found for ${c.sourceFile}`);

        usedNames.add(name);
        return { ...c, name };
      });

    const config = new RomcalConfig({
      localizedCalendar: {
        calendarName: '_',
        particularConfig: {},
        inputs: {},
        martyrology: {},
        i18n: { id: '_' },
      },
    });

    const updatedCalData: CalendarConfig[] = [];

    for (const cal of calConfigs) {
      const CalDef = cal.module[cal.name] as typeof CalendarDef;
      const calendarInstance = new CalDef(config);

      // Extract source filename and directory path for metadata generation
      const sourceFileName = path.basename(cal.sourceFile, '.ts');
      const directoryPath = path.dirname(cal.sourceFile);
      const json = serializeCalendarDef(calendarInstance, sourceFileName, directoryPath);

      // Use the calendar ID for the output filename instead of the source filename
      const calendarId = calendarInstance.calendarName;

      // Special case: if the calendar ID matches the directory name, use index.json
      const directoryName = path.basename(path.dirname(cal.outputFile));
      const outputFileName = calendarId === directoryName ? 'index.json' : `${calendarId}.json`;
      const outputFile = path.join(path.dirname(cal.outputFile), outputFileName);

      fs.writeFileSync(outputFile, JSON.stringify(json, null, 2));

      // Update the config with the new output file path for index generation
      updatedCalData.push({
        ...cal,
        outputFile,
      });
    }

    // console.log(calConfigs);

    console.log('✨ All calendar JSON files are up to date!');
  } catch (error) {
    console.error('❌ Error during calendar JSON generation:', error);
    process.exit(1);
  }
}

// Execute the script
main();
