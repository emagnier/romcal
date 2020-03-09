import { ValidationError } from 'jsonschema';
import { getDateItemSchemaValidator, getDateItemDataJsonSchema } from '@RomcalValidators/date-item.validator';
import { getRomcalConfigSchemaValidator, getRomcalConfigJsonSchema } from '@RomcalValidators/romcal-config.validator';
import { IRomcalConfig } from '@RomcalModels/romcal-config';
import { DateItem } from '@RomcalModels/romcal-date-item';

import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
dayjs.extend(utc);

/**
 * Primitive types
 */
export type Primitive = string | boolean | number | null | undefined;

/**
 * Custom date string type
 */
export type ISO8601DateString = string;

type AllowedDictonaryKeyTypes = string | number;
export type Dictionary<T> = {
  [index in AllowedDictonaryKeyTypes]: T;
};

export type TLocalizeParams = {
  key: string;
  day?: string;
  week?: number;
  count?: number;
  useDefaultOrdinalFn?: boolean;
};

/**
 * Check if the arbitary value given is an instance of [[IRomcalConfig]].
 * @param maybeRomcalConfig The value that could be an instance of [[IRomcalConfig]]
 */
export const isRomcalConfig = (maybeRomcalConfig: unknown): maybeRomcalConfig is IRomcalConfig => {
  const { errors, valid } = getRomcalConfigSchemaValidator().validate(maybeRomcalConfig, getRomcalConfigJsonSchema());
  if (!valid) {
    errors.forEach((error: ValidationError) => {
      console.error(error.name, error.property, error.message, error.stack);
    });
  }
  return valid;
};

/**
 * Check if the arbitary value given is an array of [[DateItems]].
 * @param maybeRomcalDateItems A value that could be an array of [[DateItem]]s
 */
export const areRomcalDateItems = (maybeRomcalDateItems: unknown): maybeRomcalDateItems is Array<DateItem> => {
  const { errors, valid } = getDateItemSchemaValidator().validate(maybeRomcalDateItems, getDateItemDataJsonSchema());
  if (!valid) {
    errors.forEach((error: ValidationError) => {
      console.error(error.name, error.property, error.message, error.stack);
    });
  }
  return valid;
};

/**
 * Check if a value is a valid ISO8601 Date string.
 * @param value The value to test
 */
export const isISODateString = (value: string): value is ISO8601DateString => dayjs.utc(value).isValid();

/**
 * Check if a value is *NOT* `undefined`. This is useful to check if optional props is specified.
 * @param maybeUndefined the value that could be `undefined`
 */
export const isDefined = <T>(maybeUndefined: T | undefined): maybeUndefined is T =>
  typeof maybeUndefined !== 'undefined';

/**
 * Check if a value is `undefined` or `null`.
 * @param maybeNil The value that could be `null` or `undefined`
 */
export const isNil = (maybeNil: unknown): maybeNil is undefined | null =>
  typeof maybeNil === 'undefined' || maybeNil === null;

/**
 * Check if a value is boolean (`true` / `false`).
 * @param maybeBoolean The value that could be a boolean
 */
export const isBool = (maybeBoolean: unknown): maybeBoolean is boolean => typeof maybeBoolean === 'boolean';

/**
 * Check if a value is a number.
 * @param maybeInteger The value that could be a number
 */
export const isInteger = (maybeInteger: unknown): maybeInteger is number =>
  typeof maybeInteger === 'number' && !isNil(maybeInteger);

/**
 * Check if the value is a string.
 * @param maybeString The value that could be a string
 */
export const isString = (maybeString: unknown): maybeString is string =>
  typeof maybeString === 'string' && !isNil(maybeString);

/**
 * Check if a value is primitive (`undefined`, `null`, `boolean`, `number`, or `string`).
 * @param maybePrimitive The value that could be a primitive type
 */
export const isPrimitive = (maybePrimitive: unknown): maybePrimitive is Primitive =>
  maybePrimitive === null || ['string', 'undefined', 'boolean', 'number'].indexOf(typeof maybePrimitive) !== -1;

/**
 * Check if given value is a function
 * @param maybeFunction The value that could be a function
 */
export const isFunction = (maybeFunction: unknown): maybeFunction is Function => typeof maybeFunction === 'function';

/**
 * Checks if a value is an object.
 * @param value The value that could be an object
 */
export const isObject = (maybeObject: unknown): maybeObject is object => {
  const valType = typeof maybeObject;
  return !!maybeObject && valType === 'object';
};

/**
 * Check if a value is empty.
 * @param maybeEmpty The value that could be empty
 */
export const isEmpty = (maybeEmpty: unknown): boolean => {
  if (isObject(maybeEmpty)) {
    return Object.keys(maybeEmpty).length > 0 ? false : true;
  } else {
    return true;
  }
};
