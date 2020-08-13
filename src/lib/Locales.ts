import _ from 'lodash';

import { parse, Schema } from 'bcp-47';
import { toOrdinal, toWordsOrdinal } from 'number-to-words';
import logger from '@romcal/utils/logger/logger';

import { findDescendantValueByKeys, mergeObjectsUniquely } from '@romcal/utils/object/object';
import { isNil, isString } from '@romcal/utils/type-guards/type-guards';
import { RomcalLocale } from '@romcal/models/romcal-locale/romcal-locale';
import { RomcalDateItemInput } from '@romcal/models/romcal-date-item/romcal-date-item.model';
import { Ranks } from '@romcal/constants/ranks/ranks.enum';
import { RomcalDateItemSources } from '@romcal/models/romcal-date-item/date-item-sources.type';
import { RomcalLocaleKey } from '@romcal/models/romcal-locale/locale-types.type';
import { RomcalLocalizeParams } from '@romcal/models/romcal-locale/localize-params.type';

/**
 * Load DayJS and relevant plugins
 */
import dayjs from 'dayjs';
import updateLocale from 'dayjs/plugin/updateLocale';
import advancedFormat from 'dayjs/plugin/advancedFormat';

/**
 * Extend dayjs instance with plugins
 */
dayjs.extend(updateLocale);
dayjs.extend(advancedFormat);

/**
 *  Mustache style templating is easier on the eyes
 */
_.templateSettings.interpolate = /{{([\s\S]+?)}}/g;

/**
 * Set locale
 * Locale lookup for celebration names are based on [i18n conventions as used by DayJS](https://github.com/iamkun/dayjs/tree/dev/src/locale).
 * romcal defines at least the default 'en' language as a fallback.
 * If a region is specified in the locale ('xx-XX'), romcal will
 * automatically manage a graceful fallback to its base language ('xx'), if it exists in 'src/locales'.
 * We get then a cascade fallbacks: region ('xx-XX') -> base language ('xx') -> default 'en'
 * For example: if a string is missing in 'fr-CA', it will try to pick it in 'fr', and then in 'en'.
 */
const _fallbackLocaleKey: RomcalLocaleKey = 'en';

/**
 * Cache value for the array of locales to be used for calendar output.
 */
let _locales: RomcalLocale[];

/**
 * The cache key that holds the flattened _locales array.
 */
let _combinedLocale: RomcalLocale | undefined;

/**
 * Cache value to hold the current locale's data.
 */
let _currentLocaleData: ILocale;

/**
 * Returns an ordinal number representation of the integer provided.
 *
 * Ordinal numbers look like this: 1st, 2nd, 3rd, 4th, ..., 10th and so on.
 *
 * @param value The number to process
 * @param asWord Returns the ordinal representation of the number in words only (defaults to `false`)
 */
export const ordinal = (value: number, asWord = false): string => {
  return asWord ? toWordsOrdinal(value) : toOrdinal(value);
};

/**
 * Attempts to sanitize the incoming value into a valid BCP-47 IETF locale string.
 * @param value The locale value to sanitize
 * @returns a BCP-47 IETF locale string along the [[Schema]] object with additional locale properties or throws an Error if the value could not be parsed
 */
export const sanitizePossibleLocaleValue = (
  value: string,
): {
  localeSchema: Schema;
  resolvedBcp47Locale: string;
} => {
  try {
    let localeSchema: Schema = parse(value, {
      forgiving: true, // https://www.npmjs.com/package/bcp-47#optionsforgiving
    });
    // Get schema parts
    let { region, language } = localeSchema;
    // Will hold the parsed locale
    let resolvedBcp47Locale: string;
    // Handle locales where language and region are not separated by a hyphen
    if (!isNil(language) && language.length > 2 && isNil(region)) {
      const parsedLanguage = language?.substr(0, 2);
      const parsedRegion = language?.substr(2, 4);
      resolvedBcp47Locale = `${parsedLanguage}${
        isString(parsedRegion) && parsedRegion.length > 0 ? `-${parsedRegion}` : ''
      }`;
      logger.debug(`sanitized the ${language} locale to ${resolvedBcp47Locale}`);
      localeSchema = parse(resolvedBcp47Locale, {
        forgiving: true,
      });
      region = localeSchema.region;
      language = localeSchema.language;
    } else {
      resolvedBcp47Locale = `${language}${isString(region) && region.length > 0 ? `-${region}` : ''}`;
    }
    return { localeSchema, resolvedBcp47Locale };
  } catch (e) {
    logger.warn(`${value} is not a parse-able BCP-47 IETF locale string`);
    throw e;
  }
};

/**
 * Sets the global locale for DayJS to be used for date operations throughout the library.
 * @param key The language key to use
 * @param customOrdinalFn An optional custom function to use for generating ordinal number values (defaults [[Locales.ordinal]] if not set)
 */
const setLocale = async (key: RomcalLocaleKey, customOrdinalFn: (v: number) => string = ordinal): Promise<void> => {
  // When setLocale() is called, all cache values are purged
  _combinedLocale = undefined;
  // When setLocale() is called, the cache language files are reset
  try {
    const { default: fallbackLocale } = await import(
      /* webpackExclude: /index\.ts/ */
      /* webpackChunkName: "locales/[request]" */
      /* webpackMode: "lazy" */
      `@romcal/locales/${_fallbackLocaleKey}`
    );
    _locales = [fallbackLocale as RomcalLocale];
  } catch (e) {
    logger.error(`Failed to load the ${_fallbackLocaleKey} language file`);
  }

  let currentLocale;
  try {
    const { localeSchema, resolvedBcp47Locale } = sanitizePossibleLocaleValue(key);
    const { region, language } = localeSchema;
    currentLocale = resolvedBcp47Locale;

    /**
     * If a region is specified: append the base language for that region as fallback if it exists.
     * Also check if the base language isn't already the default 'en',
     */
    if (!isNil(region) && region?.length > 0 && language !== _fallbackLocaleKey) {
      // Retrieve the relevant base locale object
      // and set it as a fallback (before 'en')
      try {
        const { default: baseLocale } = await import(
          /* webpackExclude: /index\.ts/ */
          /* webpackChunkName: "locales/[request]" */
          /* webpackMode: "lazy" */
          `@romcal/locales/${language}`
        );
        _locales = [baseLocale as RomcalLocale, ..._locales]; // For example: append the 'fr' locale
      } catch (e) {
        logger.warn(`A base language file for "${language}" to support the ${currentLocale} locale is not available`);
      }
    }

    /**
     * Finally, append the region specific locale if any to the list of locales
     */
    try {
      const { default: regionSpecificLocale } = await import(
        /* webpackExclude: /index\.ts/ */
        /* webpackChunkName: "locales/[request]" */
        /* webpackMode: "lazy" */
        `@romcal/locales/${currentLocale}`
      );
      _locales = [regionSpecificLocale as RomcalLocale, ..._locales]; // For example: append the 'fr-CA' locale
    } catch (e) {
      logger.warn(`A language file for the region locale "${currentLocale}" is not available`);
    }
  } catch (e) {
    logger.warn(`The locale "${key} is not a valid IETF BCP-47 format. romcal will default to the "en" locale`);
    currentLocale = _fallbackLocaleKey;
  }

  // Attempt to load the relevant dayjs locale configuration object
  try {
    const { default: langLocale } = await import(
      /* webpackExclude: /(index|types)\.d\.ts/ */
      /* webpackMode: "eager" */
      `dayjs/locale/${currentLocale}`
    );
    _currentLocaleData = langLocale as ILocale;
  } catch (e) {
    try {
      const languageOnly = currentLocale.split('-')[0];
      logger.warn(
        `${currentLocale} is not supported in romcal's date management library, trying to use ${languageOnly} instead`,
      );
      const { default: langLocale } = await import(
        /* webpackExclude: /(index|types)\.d\.ts/ */
        /* webpackMode: "eager" */
        `dayjs/locale/${languageOnly}`
      );
      _currentLocaleData = langLocale as ILocale;
      currentLocale = languageOnly;
    } catch (e) {
      logger.warn(`Failed to load locale data for ${currentLocale}. romcal will default to "en" locale data`);
      currentLocale = 'en';
    }
  } finally {
    // Set the locale
    dayjs.locale(currentLocale);
  }

  /**
   * Ensure that the first day is always a Sunday in romcal & DayJS
   * Monday is the first day of the week according to the international standard ISO 8601,
   * In the US, Canada, and Japan, it's counted as the second day of the week (Sunday is the first day).
   * In Christian calendars, Sunday is always the first day of the week.
   * In other words, the romcal will use US, Canada definitions for the start of the week.
   */
  dayjs.updateLocale(currentLocale, {
    weekStart: 0,
    week: {
      dow: 0, // US, Canada: 1st day of week is Sunday
      doy: 6, // US, Canada: 1st week of the year is the one that contains the 1st of January (7 + 0 - 1)
    },
    ...(isNil(_currentLocaleData.ordinal) && {
      // If the current locale's data doesn't have its own ordinal function, use this default ordinal function
      customOrdinalFn,
    }),
  });

  logger.debug(`romcal is now configured to use ${currentLocale}`);
};

/**
 * Get the current locale object.
 *
 * Return an object that combines the main locale with its fallback.
 * And use cache in case this function is called multiple times
 * without the locale being modified.
 */
const getLocale = (): RomcalLocale => {
  if (isNil(_combinedLocale)) {
    if (_locales.length > 1) {
      const [regionLocale, fallbackLocale] = _locales;
      _combinedLocale = mergeObjectsUniquely(regionLocale, fallbackLocale);
    } else {
      _combinedLocale = _locales[0];
    }
  }
  return _combinedLocale;
};

/**
 * Resolves a localized value for the given key and supporting parameters.
 *
 * Also resolves locale specific ordinal numbers where required.
 *
 * @param localizeParams Options for retrieving the localized key
 */
const localize = async ({ key, count, week, day, useDefaultOrdinalFn }: RomcalLocalizeParams): Promise<string> => {
  // Get the IETF locale set in dayjs and obtain its corresponding locale data object
  const value = findDescendantValueByKeys(getLocale(), key.split('.'));

  // Run the template against the options provided
  return _.template(value)({
    key,
    // If defined, pluralize a week and add it to the given template
    ...(typeof week === 'number' && {
      week: useDefaultOrdinalFn === true ? ordinal(week) : _currentLocaleData.ordinal?.(week) ?? ordinal(week),
    }),
    ...(typeof count === 'number' && {
      count: useDefaultOrdinalFn === true ? ordinal(count) : _currentLocaleData.ordinal?.(count) ?? ordinal(count),
    }),
    // If defined, add the day to be included in the translation
    ...(isString(day) && { day }),
  });
};

/**
 * Utility function that takes an array of national calendar dates
 * and adds a localized name based on the key.
 *
 * Allows the specification of a source where when defined, points the localization logic
 * to a specific sub-tree within the locale file to obtain localized values from.
 *
 * If the source is `temporal`, the logic will only use the key for the lookup.
 *
 * @param dates A list of [[RomcalDateItem]]s to process
 * @param source The source of the date to localize. This value is used to lookup a specific sub tree in the locale file for the localized value.
 */
const localizeDates = async (
  dates: Array<RomcalDateItemInput>,
  source: RomcalDateItemSources = 'sanctoral',
): Promise<RomcalDateItemInput[]> => {
  const promiseDates: Promise<RomcalDateItemInput>[] = dates.map(async (date: RomcalDateItemInput) => {
    return {
      ...date,
      name: await localize({
        // If the source is `temporal`, do not append anything before the date key
        key: `${source === 'temporal' ? date.key : !isNil(date.source) ? date.source : source}.${date.key}`,
      }),
    } as RomcalDateItemInput;
  });
  return await Promise.all(promiseDates);
};

/**
 * Given a "day" integer from DayJS that represents the day of week, determine
 * the rank of day from the [[Rank]] enum
 * @param day A "day" integer that should come from the DayJS library
 */
const getRankByDayOfWeek = (day: number): Ranks => (day === 0 ? Ranks.SUNDAY : Ranks.WEEKDAY);

export { setLocale, getLocale, localize, localizeDates, getRankByDayOfWeek };
