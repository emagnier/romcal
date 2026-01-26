import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type {
  CalendarDefinition,
  CalendarMetadata,
  ParticularConfig,
  DayDefinition,
  EntityDefinition,
  NavigationItem,
  EditorCalendarInfo,
  EditorEntityInfo,
} from '../types';

interface EditorState {
  // Calendar data
  calendar: CalendarDefinition;
  originalCalendar: CalendarDefinition | null;

  // Entity resources by locale
  entities: Record<string, Record<string, EntityDefinition>>;
  originalEntities: Record<string, Record<string, EntityDefinition>>;

  // Available data (loaded from repo)
  availableCalendars: EditorCalendarInfo[];
  availableEntities: EditorEntityInfo[];

  // Navigation
  activePanel: NavigationItem;

  // File System Access API
  directoryHandle: FileSystemDirectoryHandle | null;
  isConnected: boolean;
  repoPath: string | null;

  // UI State
  isDirty: boolean;
  validationErrors: Record<string, string[]>;
  isLoading: boolean;
  previewYear: number;
  isPreviewCollapsed: boolean;
}

interface EditorActions {
  // Calendar actions
  setCalendar: (calendar: CalendarDefinition) => void;
  updateMetadata: (metadata: Partial<CalendarMetadata>) => void;
  updateParticularConfig: (config: Partial<ParticularConfig>) => void;
  setParentCalendarIds: (ids: string[]) => void;
  addDayDefinition: (id: string, day: DayDefinition) => void;
  updateDayDefinition: (id: string, day: Partial<DayDefinition>) => void;
  removeDayDefinition: (id: string) => void;
  renameDayDefinition: (oldId: string, newId: string) => void;

  // Entity actions
  setEntities: (locale: string, entities: Record<string, EntityDefinition>) => void;
  addEntity: (locale: string, id: string, entity: EntityDefinition) => void;
  updateEntity: (locale: string, id: string, entity: Partial<EntityDefinition>) => void;
  removeEntity: (locale: string, id: string) => void;
  renameEntity: (locale: string, oldId: string, newId: string) => void;

  // Navigation actions
  setActivePanel: (panel: NavigationItem) => void;

  // Available data actions
  setAvailableCalendars: (calendars: EditorCalendarInfo[]) => void;
  setAvailableEntities: (entities: EditorEntityInfo[]) => void;

  // File System actions
  setDirectoryHandle: (handle: FileSystemDirectoryHandle | null) => void;
  setRepoPath: (path: string | null) => void;

  // UI actions
  setIsLoading: (loading: boolean) => void;
  setValidationErrors: (errors: Record<string, string[]>) => void;
  setPreviewYear: (year: number) => void;
  togglePreviewCollapsed: () => void;

  // Reset actions
  resetCalendar: () => void;
  createNewCalendar: () => void;
  markAsSaved: () => void;
}

const defaultCalendar: CalendarDefinition = {
  $schema: '../../../../schemas/calendar_definition.json',
  id: '',
  metadata: {
    type: 'DIOCESE',
    jurisdiction: 'ECCLESIASTICAL',
  },
  particular_config: null,
  parent_calendar_ids: ['general_roman'],
  days_definitions: {},
};

export const useEditorStore = create<EditorState & EditorActions>()(
  immer((set, get) => ({
    // Initial state
    calendar: { ...defaultCalendar },
    originalCalendar: null,
    entities: {},
    originalEntities: {},
    availableCalendars: [],
    availableEntities: [],
    activePanel: { type: 'metadata' },
    directoryHandle: null,
    isConnected: false,
    repoPath: null,
    isDirty: false,
    validationErrors: {},
    isLoading: false,
    previewYear: new Date().getFullYear(),
    isPreviewCollapsed: false,

    // Calendar actions
    setCalendar: (calendar) =>
      set((state) => {
        state.calendar = calendar;
        state.originalCalendar = JSON.parse(JSON.stringify(calendar));
        state.isDirty = false;
      }),

    updateMetadata: (metadata) =>
      set((state) => {
        state.calendar.metadata = { ...state.calendar.metadata, ...metadata };
        state.isDirty = true;
      }),

    updateParticularConfig: (config) =>
      set((state) => {
        state.calendar.particular_config = {
          ...state.calendar.particular_config,
          ...config,
        };
        state.isDirty = true;
      }),

    setParentCalendarIds: (ids) =>
      set((state) => {
        state.calendar.parent_calendar_ids = ids;
        state.isDirty = true;
      }),

    addDayDefinition: (id, day) =>
      set((state) => {
        state.calendar.days_definitions[id] = day;
        state.isDirty = true;
      }),

    updateDayDefinition: (id, day) =>
      set((state) => {
        if (state.calendar.days_definitions[id]) {
          state.calendar.days_definitions[id] = {
            ...state.calendar.days_definitions[id],
            ...day,
          };
          state.isDirty = true;
        }
      }),

    removeDayDefinition: (id) =>
      set((state) => {
        delete state.calendar.days_definitions[id];
        state.isDirty = true;
      }),

    renameDayDefinition: (oldId, newId) =>
      set((state) => {
        if (state.calendar.days_definitions[oldId] && oldId !== newId) {
          state.calendar.days_definitions[newId] = state.calendar.days_definitions[oldId];
          delete state.calendar.days_definitions[oldId];
          state.isDirty = true;
        }
      }),

    // Entity actions
    setEntities: (locale, entities) =>
      set((state) => {
        state.entities[locale] = entities;
        state.originalEntities[locale] = JSON.parse(JSON.stringify(entities));
      }),

    addEntity: (locale, id, entity) =>
      set((state) => {
        if (!state.entities[locale]) {
          state.entities[locale] = {};
        }
        state.entities[locale][id] = entity;
        state.isDirty = true;
      }),

    updateEntity: (locale, id, entity) =>
      set((state) => {
        if (state.entities[locale]?.[id]) {
          state.entities[locale][id] = {
            ...state.entities[locale][id],
            ...entity,
          };
          state.isDirty = true;
        }
      }),

    removeEntity: (locale, id) =>
      set((state) => {
        if (state.entities[locale]) {
          delete state.entities[locale][id];
          state.isDirty = true;
        }
      }),

    renameEntity: (locale, oldId, newId) =>
      set((state) => {
        if (state.entities[locale]?.[oldId] && oldId !== newId) {
          state.entities[locale][newId] = state.entities[locale][oldId];
          delete state.entities[locale][oldId];
          state.isDirty = true;
        }
      }),

    // Navigation actions
    setActivePanel: (panel) =>
      set((state) => {
        state.activePanel = panel;
      }),

    // Available data actions
    setAvailableCalendars: (calendars) =>
      set((state) => {
        state.availableCalendars = calendars;
      }),

    setAvailableEntities: (entities) =>
      set((state) => {
        state.availableEntities = entities;
      }),

    // File System actions
    setDirectoryHandle: (handle) =>
      set((state) => {
        state.directoryHandle = handle;
        state.isConnected = handle !== null;
      }),

    setRepoPath: (path) =>
      set((state) => {
        state.repoPath = path;
      }),

    // UI actions
    setIsLoading: (loading) =>
      set((state) => {
        state.isLoading = loading;
      }),

    setValidationErrors: (errors) =>
      set((state) => {
        state.validationErrors = errors;
      }),

    setPreviewYear: (year) =>
      set((state) => {
        state.previewYear = year;
      }),

    togglePreviewCollapsed: () =>
      set((state) => {
        state.isPreviewCollapsed = !state.isPreviewCollapsed;
      }),

    // Reset actions
    resetCalendar: () =>
      set((state) => {
        if (state.originalCalendar) {
          state.calendar = JSON.parse(JSON.stringify(state.originalCalendar));
          state.isDirty = false;
        }
      }),

    createNewCalendar: () =>
      set((state) => {
        state.calendar = { ...defaultCalendar };
        state.originalCalendar = null;
        state.entities = {};
        state.originalEntities = {};
        state.isDirty = false;
        state.validationErrors = {};
        state.activePanel = { type: 'metadata' };
      }),

    markAsSaved: () =>
      set((state) => {
        state.originalCalendar = JSON.parse(JSON.stringify(state.calendar));
        state.originalEntities = JSON.parse(JSON.stringify(state.entities));
        state.isDirty = false;
      }),
  }))
);

// Selectors
export const selectDayCount = (state: EditorState) => Object.keys(state.calendar.days_definitions).length;

export const selectEntityCount = (state: EditorState, locale: string) =>
  Object.keys(state.entities[locale] || {}).length;

export const selectHasUnsavedChanges = (state: EditorState) => state.isDirty;

export const selectIsFileSystemSupported = () => typeof window !== 'undefined' && 'showDirectoryPicker' in window;
