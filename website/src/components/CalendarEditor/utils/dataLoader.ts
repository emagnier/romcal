/**
 * Data loader utilities for the Calendar Editor
 * Loads available calendars and entities from static data
 */

import type {
  EditorCalendarInfo,
  EditorEntityInfo,
  CalendarDefinition,
  EntityDefinition,
  CalendarType,
} from '../types';

// Static data for available calendars (will be populated at build time)
// For now, we define a minimal set
const CALENDAR_DATA: EditorCalendarInfo[] = [
  {
    id: 'general_roman',
    name: 'General Roman Calendar',
    type: 'GENERAL_ROMAN',
    path: 'general_roman/general_roman.json',
  },
  { id: 'africa', name: 'Africa', type: 'REGION', path: 'regions/africa.json' },
  { id: 'americas', name: 'Americas', type: 'REGION', path: 'regions/americas.json' },
  { id: 'asia', name: 'Asia', type: 'REGION', path: 'regions/asia.json' },
  { id: 'europe', name: 'Europe', type: 'REGION', path: 'regions/europe.json' },
  { id: 'argentina', name: 'Argentina', type: 'COUNTRY', path: 'countries/argentina/argentina.json' },
  { id: 'australia', name: 'Australia', type: 'COUNTRY', path: 'countries/australia/australia.json' },
  { id: 'austria', name: 'Austria', type: 'COUNTRY', path: 'countries/austria/austria.json' },
  { id: 'belgium', name: 'Belgium', type: 'COUNTRY', path: 'countries/belgium/belgium.json' },
  { id: 'bolivia', name: 'Bolivia', type: 'COUNTRY', path: 'countries/bolivia/bolivia.json' },
  {
    id: 'bosnia_herzegovina',
    name: 'Bosnia Herzegovina',
    type: 'COUNTRY',
    path: 'countries/bosnia_herzegovina/bosnia_herzegovina.json',
  },
  { id: 'brazil', name: 'Brazil', type: 'COUNTRY', path: 'countries/brazil/brazil.json' },
  { id: 'canada', name: 'Canada', type: 'COUNTRY', path: 'countries/canada/canada.json' },
  { id: 'chile', name: 'Chile', type: 'COUNTRY', path: 'countries/chile/chile.json' },
  { id: 'china', name: 'China', type: 'COUNTRY', path: 'countries/china/china.json' },
  { id: 'costa_rica', name: 'Costa Rica', type: 'COUNTRY', path: 'countries/costa_rica/costa_rica.json' },
  { id: 'croatia', name: 'Croatia', type: 'COUNTRY', path: 'countries/croatia/croatia.json' },
  {
    id: 'czech_republic',
    name: 'Czech Republic',
    type: 'COUNTRY',
    path: 'countries/czech_republic/czech_republic.json',
  },
  { id: 'denmark', name: 'Denmark', type: 'COUNTRY', path: 'countries/denmark/denmark.json' },
  { id: 'england', name: 'England', type: 'COUNTRY', path: 'countries/england/england.json' },
  { id: 'finland', name: 'Finland', type: 'COUNTRY', path: 'countries/finland/finland.json' },
  { id: 'france', name: 'France', type: 'COUNTRY', path: 'countries/france/france.json' },
  { id: 'france__angers', name: 'France - Angers', type: 'DIOCESE', path: 'countries/france/france__angers.json' },
  {
    id: 'france__coutances',
    name: 'France - Coutances',
    type: 'DIOCESE',
    path: 'countries/france/france__coutances.json',
  },
  { id: 'france__lyon', name: 'France - Lyon', type: 'DIOCESE', path: 'countries/france/france__lyon.json' },
  { id: 'france__paris', name: 'France - Paris', type: 'DIOCESE', path: 'countries/france/france__paris.json' },
  {
    id: 'france__saint_denis',
    name: 'France - Saint-Denis',
    type: 'DIOCESE',
    path: 'countries/france/france__saint_denis.json',
  },
  {
    id: 'france__strasbourg',
    name: 'France - Strasbourg',
    type: 'DIOCESE',
    path: 'countries/france/france__strasbourg.json',
  },
  {
    id: 'france__toulouse',
    name: 'France - Toulouse',
    type: 'DIOCESE',
    path: 'countries/france/france__toulouse.json',
  },
  { id: 'germany', name: 'Germany', type: 'COUNTRY', path: 'countries/germany/germany.json' },
  { id: 'greece', name: 'Greece', type: 'COUNTRY', path: 'countries/greece/greece.json' },
  { id: 'guatemala', name: 'Guatemala', type: 'COUNTRY', path: 'countries/guatemala/guatemala.json' },
  { id: 'hungary', name: 'Hungary', type: 'COUNTRY', path: 'countries/hungary/hungary.json' },
  { id: 'india', name: 'India', type: 'COUNTRY', path: 'countries/india/india.json' },
  { id: 'ireland', name: 'Ireland', type: 'COUNTRY', path: 'countries/ireland/ireland.json' },
  { id: 'italy', name: 'Italy', type: 'COUNTRY', path: 'countries/italy/italy.json' },
  { id: 'japan', name: 'Japan', type: 'COUNTRY', path: 'countries/japan/japan.json' },
  { id: 'lebanon', name: 'Lebanon', type: 'COUNTRY', path: 'countries/lebanon/lebanon.json' },
  { id: 'lithuania', name: 'Lithuania', type: 'COUNTRY', path: 'countries/lithuania/lithuania.json' },
  { id: 'malta', name: 'Malta', type: 'COUNTRY', path: 'countries/malta/malta.json' },
  { id: 'mexico', name: 'Mexico', type: 'COUNTRY', path: 'countries/mexico/mexico.json' },
  { id: 'netherlands', name: 'Netherlands', type: 'COUNTRY', path: 'countries/netherlands/netherlands.json' },
  { id: 'new_zealand', name: 'New Zealand', type: 'COUNTRY', path: 'countries/new_zealand/new_zealand.json' },
  { id: 'norway', name: 'Norway', type: 'COUNTRY', path: 'countries/norway/norway.json' },
  { id: 'panama', name: 'Panama', type: 'COUNTRY', path: 'countries/panama/panama.json' },
  { id: 'paraguay', name: 'Paraguay', type: 'COUNTRY', path: 'countries/paraguay/paraguay.json' },
  { id: 'peru', name: 'Peru', type: 'COUNTRY', path: 'countries/peru/peru.json' },
  { id: 'philippines', name: 'Philippines', type: 'COUNTRY', path: 'countries/philippines/philippines.json' },
  { id: 'poland', name: 'Poland', type: 'COUNTRY', path: 'countries/poland/poland.json' },
  { id: 'portugal', name: 'Portugal', type: 'COUNTRY', path: 'countries/portugal/portugal.json' },
  { id: 'puerto_rico', name: 'Puerto Rico', type: 'COUNTRY', path: 'countries/puerto_rico/puerto_rico.json' },
  { id: 'romania', name: 'Romania', type: 'COUNTRY', path: 'countries/romania/romania.json' },
  { id: 'russia', name: 'Russia', type: 'COUNTRY', path: 'countries/russia/russia.json' },
  { id: 'scotland', name: 'Scotland', type: 'COUNTRY', path: 'countries/scotland/scotland.json' },
  { id: 'slovakia', name: 'Slovakia', type: 'COUNTRY', path: 'countries/slovakia/slovakia.json' },
  { id: 'slovenia', name: 'Slovenia', type: 'COUNTRY', path: 'countries/slovenia/slovenia.json' },
  { id: 'spain', name: 'Spain', type: 'COUNTRY', path: 'countries/spain/spain.json' },
  { id: 'sri_lanka', name: 'Sri Lanka', type: 'COUNTRY', path: 'countries/sri_lanka/sri_lanka.json' },
  { id: 'sweden', name: 'Sweden', type: 'COUNTRY', path: 'countries/sweden/sweden.json' },
  { id: 'switzerland', name: 'Switzerland', type: 'COUNTRY', path: 'countries/switzerland/switzerland.json' },
  { id: 'ukraine', name: 'Ukraine', type: 'COUNTRY', path: 'countries/ukraine/ukraine.json' },
  { id: 'united_states', name: 'United States', type: 'COUNTRY', path: 'countries/united_states/united_states.json' },
  { id: 'uruguay', name: 'Uruguay', type: 'COUNTRY', path: 'countries/uruguay/uruguay.json' },
  { id: 'venezuela', name: 'Venezuela', type: 'COUNTRY', path: 'countries/venezuela/venezuela.json' },
  { id: 'vietnam', name: 'Vietnam', type: 'COUNTRY', path: 'countries/vietnam/vietnam.json' },
  { id: 'wales', name: 'Wales', type: 'COUNTRY', path: 'countries/wales/wales.json' },
];

// Available locales
export const AVAILABLE_LOCALES = [
  { code: 'cs', name: 'Czech' },
  { code: 'de', name: 'German' },
  { code: 'en', name: 'English' },
  { code: 'en-gb', name: 'English (UK)' },
  { code: 'en-ie', name: 'English (Ireland)' },
  { code: 'es', name: 'Spanish' },
  { code: 'fr', name: 'French' },
  { code: 'it', name: 'Italian' },
  { code: 'la', name: 'Latin' },
  { code: 'pl', name: 'Polish' },
  { code: 'pt-br', name: 'Portuguese (Brazil)' },
  { code: 'sk', name: 'Slovak' },
  { code: 'ta', name: 'Tamil' },
];

/**
 * Load available calendars
 */
export async function loadAvailableCalendars(): Promise<EditorCalendarInfo[]> {
  // In a production build, this would load from static JSON
  // For now, return the static data
  return CALENDAR_DATA;
}

/**
 * Load available entities (summary only)
 */
export async function loadAvailableEntities(): Promise<EditorEntityInfo[]> {
  // In a production build, this would load from static JSON
  // For now, return an empty array - entities will be loaded on demand
  return [];
}

/**
 * Group calendars by type
 */
export function groupCalendarsByType(calendars: EditorCalendarInfo[]): Record<CalendarType, EditorCalendarInfo[]> {
  const groups: Record<CalendarType, EditorCalendarInfo[]> = {
    GENERAL_ROMAN: [],
    REGION: [],
    COUNTRY: [],
    ARCHDIOCESE: [],
    DIOCESE: [],
    CITY: [],
    PARISH: [],
    GENERAL_COMMUNITY: [],
    REGIONAL_COMMUNITY: [],
    LOCAL_COMMUNITY: [],
    OTHER: [],
  };

  for (const calendar of calendars) {
    groups[calendar.type].push(calendar);
  }

  return groups;
}

/**
 * Get display name for calendar type
 */
export function getCalendarTypeLabel(type: CalendarType): string {
  const labels: Record<CalendarType, string> = {
    GENERAL_ROMAN: 'General Roman',
    REGION: 'Region',
    COUNTRY: 'Country',
    ARCHDIOCESE: 'Archdiocese',
    DIOCESE: 'Diocese',
    CITY: 'City',
    PARISH: 'Parish',
    GENERAL_COMMUNITY: 'General Community',
    REGIONAL_COMMUNITY: 'Regional Community',
    LOCAL_COMMUNITY: 'Local Community',
    OTHER: 'Other',
  };
  return labels[type];
}

/**
 * Load calendar definition from File System API
 */
export async function loadCalendarFromFileSystem(
  dirHandle: FileSystemDirectoryHandle,
  calendarPath: string
): Promise<CalendarDefinition | null> {
  try {
    const pathParts = calendarPath.split('/');
    let currentHandle: FileSystemDirectoryHandle = dirHandle;

    // Navigate to data/definitions folder
    for (const folder of ['data', 'definitions']) {
      currentHandle = await currentHandle.getDirectoryHandle(folder);
    }

    // Navigate to the calendar file
    for (let i = 0; i < pathParts.length - 1; i++) {
      currentHandle = await currentHandle.getDirectoryHandle(pathParts[i]);
    }

    const fileName = pathParts[pathParts.length - 1];
    const fileHandle = await currentHandle.getFileHandle(fileName);
    const file = await fileHandle.getFile();
    const text = await file.text();

    return JSON.parse(text) as CalendarDefinition;
  } catch (error) {
    console.error('Failed to load calendar from file system:', error);
    return null;
  }
}

/**
 * Load entities from File System API
 */
export async function loadEntitiesFromFileSystem(
  dirHandle: FileSystemDirectoryHandle,
  locale: string
): Promise<Record<string, EntityDefinition>> {
  const entities: Record<string, EntityDefinition> = {};

  try {
    let currentHandle: FileSystemDirectoryHandle = dirHandle;

    // Navigate to data/resources/locale folder
    for (const folder of ['data', 'resources', locale]) {
      currentHandle = await currentHandle.getDirectoryHandle(folder);
    }

    // Read all entity files (entities.a.json through entities.z.json)
    for await (const entry of currentHandle.values()) {
      if (entry.kind === 'file' && entry.name.startsWith('entities.') && entry.name.endsWith('.json')) {
        const fileHandle = await currentHandle.getFileHandle(entry.name);
        const file = await fileHandle.getFile();
        const text = await file.text();
        const data = JSON.parse(text);

        if (data.entities) {
          Object.assign(entities, data.entities);
        }
      }
    }
  } catch (error) {
    console.error(`Failed to load entities for locale ${locale}:`, error);
  }

  return entities;
}

/**
 * Generate a valid calendar ID from name
 */
export function generateCalendarId(name: string, parentId?: string): string {
  const baseName = name
    .toLowerCase()
    .replace(/[^a-z0-9\s]/g, '')
    .replace(/\s+/g, '_')
    .replace(/_+/g, '_')
    .trim();

  if (parentId && !baseName.startsWith(parentId)) {
    return `${parentId}__${baseName}`;
  }

  return baseName;
}

/**
 * Generate a valid entity ID from name
 */
export function generateEntityId(name: string, titles?: string[]): string {
  let id = name
    .toLowerCase()
    .replace(/[^a-z0-9\s]/g, '')
    .replace(/\s+/g, '_')
    .replace(/_+/g, '_')
    .trim();

  if (titles && titles.length > 0) {
    const titleSuffix = titles.map((t) => t.toLowerCase()).join('_');
    id = `${id}_${titleSuffix}`;
  }

  return id;
}

/**
 * Get the first letter from an entity ID for file organization
 */
export function getEntityFileLetter(entityId: string): string {
  const firstLetter = entityId.charAt(0).toLowerCase();
  return /[a-z]/.test(firstLetter) ? firstLetter : 'other';
}

/**
 * Scan the repository for available calendars
 */
export async function scanRepositoryCalendars(dirHandle: FileSystemDirectoryHandle): Promise<EditorCalendarInfo[]> {
  const calendars: EditorCalendarInfo[] = [];

  try {
    let definitionsHandle: FileSystemDirectoryHandle = dirHandle;

    // Navigate to data/definitions folder
    for (const folder of ['data', 'definitions']) {
      definitionsHandle = await definitionsHandle.getDirectoryHandle(folder);
    }

    // Scan general_roman
    try {
      const generalHandle = await definitionsHandle.getDirectoryHandle('general_roman');
      const calendar = await scanCalendarFile(generalHandle, 'general_roman.json', 'general_roman');
      if (calendar) {
        calendars.push(calendar);
      }
    } catch {
      // general_roman not found
    }

    // Scan regions
    try {
      const regionsHandle = await definitionsHandle.getDirectoryHandle('regions');
      for await (const entry of regionsHandle.values()) {
        if (entry.kind === 'file' && entry.name.endsWith('.json')) {
          const calendar = await scanCalendarFile(regionsHandle, entry.name, `regions/${entry.name}`);
          if (calendar) {
            calendars.push(calendar);
          }
        }
      }
    } catch {
      // regions folder not found
    }

    // Scan countries
    try {
      const countriesHandle = await definitionsHandle.getDirectoryHandle('countries');
      for await (const countryEntry of countriesHandle.values()) {
        if (countryEntry.kind === 'directory') {
          const countryDirHandle = await countriesHandle.getDirectoryHandle(countryEntry.name);
          for await (const fileEntry of countryDirHandle.values()) {
            if (fileEntry.kind === 'file' && fileEntry.name.endsWith('.json')) {
              const calendar = await scanCalendarFile(
                countryDirHandle,
                fileEntry.name,
                `countries/${countryEntry.name}/${fileEntry.name}`
              );
              if (calendar) {
                calendars.push(calendar);
              }
            }
          }
        }
      }
    } catch {
      // countries folder not found
    }
  } catch (error) {
    console.error('Failed to scan repository calendars:', error);
  }

  return calendars.sort((a, b) => a.id.localeCompare(b.id));
}

async function scanCalendarFile(
  dirHandle: FileSystemDirectoryHandle,
  fileName: string,
  path: string
): Promise<EditorCalendarInfo | null> {
  try {
    const fileHandle = await dirHandle.getFileHandle(fileName);
    const file = await fileHandle.getFile();
    const text = await file.text();
    const data = JSON.parse(text) as CalendarDefinition;

    return {
      id: data.id,
      name: formatCalendarName(data.id),
      type: data.metadata.type,
      path,
    };
  } catch {
    return null;
  }
}

function formatCalendarName(id: string): string {
  return id
    .replace(/__/g, ' - ')
    .replace(/_/g, ' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
}
