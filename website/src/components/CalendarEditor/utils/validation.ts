/**
 * Validation utilities for the Calendar Editor
 * Uses Ajv for JSON Schema validation
 */

import Ajv from 'ajv';
import type { CalendarDefinition, EntityDefinition, DayDefinition } from '../types';

// Create Ajv instance
const ajv = new Ajv({
  allErrors: true,
  verbose: true,
  strict: false,
});

// Simplified schemas for client-side validation
// Full validation should be done against the complete schemas

const calendarIdPattern = /^[a-z][a-z0-9_]*(__[a-z][a-z0-9_]*)*$/;
const entityIdPattern = /^[a-z][a-z0-9_]*$/;

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export interface ValidationError {
  path: string;
  message: string;
  severity: 'error' | 'warning';
}

/**
 * Validate calendar ID format
 */
export function validateCalendarId(id: string): ValidationResult {
  const errors: ValidationError[] = [];

  if (!id) {
    errors.push({
      path: 'id',
      message: 'Calendar ID is required',
      severity: 'error',
    });
  } else if (!calendarIdPattern.test(id)) {
    errors.push({
      path: 'id',
      message: 'Calendar ID must be snake_case (use double underscore for hierarchy)',
      severity: 'error',
    });
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}

/**
 * Validate entity ID format
 */
export function validateEntityId(id: string): ValidationResult {
  const errors: ValidationError[] = [];

  if (!id) {
    errors.push({
      path: 'id',
      message: 'Entity ID is required',
      severity: 'error',
    });
  } else if (!entityIdPattern.test(id)) {
    errors.push({
      path: 'id',
      message: 'Entity ID must be snake_case',
      severity: 'error',
    });
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}

/**
 * Validate calendar metadata
 */
export function validateCalendarMetadata(calendar: CalendarDefinition): ValidationResult {
  const errors: ValidationError[] = [];

  // ID validation
  const idResult = validateCalendarId(calendar.id);
  errors.push(...idResult.errors);

  // Metadata type
  if (!calendar.metadata?.type) {
    errors.push({
      path: 'metadata.type',
      message: 'Calendar type is required',
      severity: 'error',
    });
  }

  // Metadata jurisdiction
  if (!calendar.metadata?.jurisdiction) {
    errors.push({
      path: 'metadata.jurisdiction',
      message: 'Calendar jurisdiction is required',
      severity: 'error',
    });
  }

  // Parent calendars
  if (!calendar.parent_calendar_ids || calendar.parent_calendar_ids.length === 0) {
    if (calendar.metadata?.type !== 'GENERAL_ROMAN') {
      errors.push({
        path: 'parent_calendar_ids',
        message: 'At least one parent calendar is required (except for General Roman)',
        severity: 'warning',
      });
    }
  }

  return {
    valid: errors.filter((e) => e.severity === 'error').length === 0,
    errors,
  };
}

/**
 * Validate day definition
 */
export function validateDayDefinition(id: string, day: DayDefinition): ValidationResult {
  const errors: ValidationError[] = [];

  // ID validation
  if (!id) {
    errors.push({
      path: 'id',
      message: 'Day definition ID is required',
      severity: 'error',
    });
  } else if (!entityIdPattern.test(id)) {
    errors.push({
      path: 'id',
      message: 'Day definition ID must be snake_case',
      severity: 'error',
    });
  }

  // Date definition
  if (!day.date_def && !day.drop) {
    // Date is optional if overriding a parent calendar day
    // or if the day inherits from proper of time
  }

  // Precedence
  if (!day.precedence && !day.drop) {
    errors.push({
      path: `${id}.precedence`,
      message: 'Precedence is recommended for day definitions',
      severity: 'warning',
    });
  }

  // Date validation for fixed dates
  if (day.date_def && 'month' in day.date_def && 'date' in day.date_def) {
    const { month, date } = day.date_def;
    if (month < 1 || month > 12) {
      errors.push({
        path: `${id}.date_def.month`,
        message: 'Month must be between 1 and 12',
        severity: 'error',
      });
    }
    if (date < 1 || date > 31) {
      errors.push({
        path: `${id}.date_def.date`,
        message: 'Date must be between 1 and 31',
        severity: 'error',
      });
    }
  }

  return {
    valid: errors.filter((e) => e.severity === 'error').length === 0,
    errors,
  };
}

/**
 * Validate entity definition
 */
export function validateEntityDefinition(id: string, entity: EntityDefinition): ValidationResult {
  const errors: ValidationError[] = [];

  // ID validation
  const idResult = validateEntityId(id);
  errors.push(...idResult.errors.map((e) => ({ ...e, path: `${id}.${e.path}` })));

  // Fullname or name required
  if (!entity.fullname && !entity.name) {
    errors.push({
      path: `${id}`,
      message: 'Entity must have either fullname or name',
      severity: 'warning',
    });
  }

  return {
    valid: errors.filter((e) => e.severity === 'error').length === 0,
    errors,
  };
}

/**
 * Validate entire calendar definition
 */
export function validateCalendar(calendar: CalendarDefinition): ValidationResult {
  const errors: ValidationError[] = [];

  // Metadata validation
  const metadataResult = validateCalendarMetadata(calendar);
  errors.push(...metadataResult.errors);

  // Day definitions validation
  for (const [id, day] of Object.entries(calendar.days_definitions)) {
    const dayResult = validateDayDefinition(id, day);
    errors.push(...dayResult.errors);
  }

  return {
    valid: errors.filter((e) => e.severity === 'error').length === 0,
    errors,
  };
}

/**
 * Check for semantic warnings (entity references that don't exist, etc.)
 */
export function checkSemanticWarnings(
  calendar: CalendarDefinition,
  availableEntities: Set<string>
): ValidationError[] {
  const warnings: ValidationError[] = [];

  for (const [dayId, day] of Object.entries(calendar.days_definitions)) {
    if (day.entities) {
      for (const entityRef of day.entities) {
        const entityId = typeof entityRef === 'string' ? entityRef : entityRef.id;
        if (!availableEntities.has(entityId)) {
          warnings.push({
            path: `${dayId}.entities`,
            message: `Entity "${entityId}" not found in catalog`,
            severity: 'warning',
          });
        }
      }
    }
  }

  return warnings;
}

/**
 * Format validation errors for display
 */
export function formatValidationErrors(errors: ValidationError[]): Record<string, string[]> {
  const formatted: Record<string, string[]> = {};

  for (const error of errors) {
    if (!formatted[error.path]) {
      formatted[error.path] = [];
    }
    formatted[error.path].push(error.message);
  }

  return formatted;
}
