/**
 * JSON export utilities for the Calendar Editor
 * Handles file generation and download
 */

import type { CalendarDefinition, EntityDefinition, Resources } from '../types';
import { getEntityFileLetter } from './dataLoader';

/**
 * Generate clean JSON for calendar definition
 */
export function generateCalendarJson(calendar: CalendarDefinition): string {
  // Clean up null values and empty objects
  const cleaned = cleanObject({
    $schema: calendar.$schema || '../../../../schemas/calendar_definition.json',
    id: calendar.id,
    metadata: calendar.metadata,
    particular_config: calendar.particular_config,
    parent_calendar_ids: calendar.parent_calendar_ids,
    days_definitions: calendar.days_definitions,
  });

  return JSON.stringify(cleaned, null, 2);
}

/**
 * Generate clean JSON for entity resources
 */
export function generateEntitiesJson(
  locale: string,
  entities: Record<string, EntityDefinition>,
  letter: string
): string {
  const cleaned = cleanObject({
    $schema: '../../../schemas/resources.json',
    locale,
    entities,
  });

  return JSON.stringify(cleaned, null, 2);
}

/**
 * Group entities by first letter for file organization
 */
export function groupEntitiesByLetter(
  entities: Record<string, EntityDefinition>
): Record<string, Record<string, EntityDefinition>> {
  const groups: Record<string, Record<string, EntityDefinition>> = {};

  for (const [id, entity] of Object.entries(entities)) {
    const letter = getEntityFileLetter(id);
    if (!groups[letter]) {
      groups[letter] = {};
    }
    groups[letter][id] = entity;
  }

  return groups;
}

/**
 * Clean object by removing null, undefined, and empty values
 */
function cleanObject<T>(obj: T): T {
  if (obj === null || obj === undefined) {
    return obj;
  }

  if (Array.isArray(obj)) {
    return obj.map(cleanObject).filter((item) => item !== null && item !== undefined) as T;
  }

  if (typeof obj === 'object') {
    const cleaned: Record<string, unknown> = {};

    for (const [key, value] of Object.entries(obj as Record<string, unknown>)) {
      const cleanedValue = cleanObject(value);

      // Skip null, undefined, and empty arrays/objects
      if (cleanedValue === null || cleanedValue === undefined) continue;
      if (Array.isArray(cleanedValue) && cleanedValue.length === 0) continue;
      if (typeof cleanedValue === 'object' && Object.keys(cleanedValue).length === 0) continue;

      cleaned[key] = cleanedValue;
    }

    return cleaned as T;
  }

  return obj;
}

/**
 * Trigger browser download for a file
 */
export function downloadFile(content: string, filename: string, mimeType = 'application/json') {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');

  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

/**
 * Download calendar definition as JSON
 */
export function downloadCalendar(calendar: CalendarDefinition) {
  const json = generateCalendarJson(calendar);
  const filename = `${calendar.id}.json`;
  downloadFile(json, filename);
}

/**
 * Download all entity files for a locale
 */
export function downloadEntities(
  locale: string,
  entities: Record<string, EntityDefinition>
) {
  const groups = groupEntitiesByLetter(entities);

  for (const [letter, letterEntities] of Object.entries(groups)) {
    if (Object.keys(letterEntities).length > 0) {
      const json = generateEntitiesJson(locale, letterEntities, letter);
      const filename = `entities.${letter}.json`;
      downloadFile(json, filename);
    }
  }
}

/**
 * Save calendar to File System API
 */
export async function saveCalendarToFileSystem(
  dirHandle: FileSystemDirectoryHandle,
  calendar: CalendarDefinition,
  calendarPath: string
): Promise<boolean> {
  try {
    const pathParts = calendarPath.split('/');
    let currentHandle: FileSystemDirectoryHandle = dirHandle;

    // Navigate to data/definitions folder
    for (const folder of ['data', 'definitions']) {
      currentHandle = await currentHandle.getDirectoryHandle(folder, { create: true });
    }

    // Navigate to the calendar folder, creating if necessary
    for (let i = 0; i < pathParts.length - 1; i++) {
      currentHandle = await currentHandle.getDirectoryHandle(pathParts[i], { create: true });
    }

    // Write the file
    const fileName = pathParts[pathParts.length - 1];
    const fileHandle = await currentHandle.getFileHandle(fileName, { create: true });
    const writable = await fileHandle.createWritable();
    const json = generateCalendarJson(calendar);

    await writable.write(json);
    await writable.close();

    return true;
  } catch (error) {
    console.error('Failed to save calendar to file system:', error);
    return false;
  }
}

/**
 * Save entities to File System API
 */
export async function saveEntitiesToFileSystem(
  dirHandle: FileSystemDirectoryHandle,
  locale: string,
  entities: Record<string, EntityDefinition>
): Promise<boolean> {
  try {
    let currentHandle: FileSystemDirectoryHandle = dirHandle;

    // Navigate to data/resources/locale folder
    for (const folder of ['data', 'resources', locale]) {
      currentHandle = await currentHandle.getDirectoryHandle(folder, { create: true });
    }

    // Group entities by letter and save each file
    const groups = groupEntitiesByLetter(entities);

    for (const [letter, letterEntities] of Object.entries(groups)) {
      if (Object.keys(letterEntities).length > 0) {
        const fileName = `entities.${letter}.json`;
        const fileHandle = await currentHandle.getFileHandle(fileName, { create: true });
        const writable = await fileHandle.createWritable();
        const json = generateEntitiesJson(locale, letterEntities, letter);

        await writable.write(json);
        await writable.close();
      }
    }

    return true;
  } catch (error) {
    console.error('Failed to save entities to file system:', error);
    return false;
  }
}

/**
 * Get the expected file path for a calendar based on its ID and type
 */
export function getCalendarFilePath(id: string, type: string): string {
  const parts = id.split('__');

  if (type === 'GENERAL_ROMAN') {
    return `general_roman/${id}.json`;
  }

  if (type === 'REGION') {
    return `regions/${id}.json`;
  }

  // For countries and below, organize by country
  const countryId = parts[0];

  if (type === 'COUNTRY') {
    return `countries/${countryId}/${id}.json`;
  }

  // Diocese, City, Parish, etc.
  return `countries/${countryId}/${id}.json`;
}

/**
 * Get a list of files that would be modified by saving
 */
export function getModifiedFiles(
  calendar: CalendarDefinition,
  entities: Record<string, Record<string, EntityDefinition>>
): { path: string; type: 'calendar' | 'entities' }[] {
  const files: { path: string; type: 'calendar' | 'entities' }[] = [];

  // Calendar file
  const calendarPath = getCalendarFilePath(calendar.id, calendar.metadata.type);
  files.push({ path: `data/definitions/${calendarPath}`, type: 'calendar' });

  // Entity files
  for (const [locale, localeEntities] of Object.entries(entities)) {
    const groups = groupEntitiesByLetter(localeEntities);
    for (const letter of Object.keys(groups)) {
      if (Object.keys(groups[letter]).length > 0) {
        files.push({ path: `data/resources/${locale}/entities.${letter}.json`, type: 'entities' });
      }
    }
  }

  return files;
}
