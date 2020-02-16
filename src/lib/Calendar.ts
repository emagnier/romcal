/* eslint-disable @typescript-eslint/no-explicit-any */
import _ from "lodash";
import { utc, Moment } from "moment";
import * as CountryCalendars from "../calendars";
import Config, { IRomcalConfig } from "../models/romcal-config";
import * as Dates from "./Dates";
import * as Utils from "./Utils";
import * as Seasons from "./Seasons";
import * as Celebrations from "./Celebrations";
import { TCountryTypes, isNil, Primitive, isInteger, isObject, TRomcalQueryResult, TRomcalQuery } from "../utils/type-guards";
import { filter, hasKey, getValueByKey } from "../utils/object";
import { DateItem, IRomcalDateItem, IRomcalDateItemData, isDateItem } from "../models/romcal-date-item";
import { Types } from "../constants";

/**
 * Calendar Class:
 * Combine together all the different collections of date item objects,
 * according to the liturgical calendar for the specific year.
 */
class Calendar {
    private dateItems: Array<DateItem> = [];
    private config: Config;
    private startDate: Moment;
    private endDate: Moment;

    /**
     * Create a new Calendar
     */
    constructor(config: Config) {
        this.config = config;
        const { type, year } = config;
        // The year can be specified either as:
        // - civil year (January 1 to December 31); or
        // - liturgical year (1st Sunday of Advent to the Saturday after the Christ the King Sunday).
        if (type === "liturgical") {
            this.startDate = Dates.firstSundayOfAdvent(year);
            this.endDate = Dates.firstSundayOfAdvent(year + 1).subtract(1, "days");
        } else {
            this.startDate = utc({ year, month: 0, day: 1 });
            this.endDate = utc({ year, month: 11, day: 31 });
        }
    }

    /**
     * Fetch calendars and date items that occur during a specific year (civil or liturgical).
     */
    fetch(): Calendar {
        const {
            ascensionOnSunday,
            christmastideEnds,
            christmastideIncludesTheSeasonOfEpiphany,
            corpusChristiOnThursday,
            country,
            epiphanyOnJan6,
            type,
            year,
        } = this.config;
        const years = type === "liturgical" ? [year, year + 1] : [year];

        // Define all calendars
        let weekdayDates: Array<IRomcalDateItem> = [];
        let celebrationsDates: Array<IRomcalDateItem> = [];
        let generalDates: Array<IRomcalDateItem> = [];
        let nationalDates: Array<IRomcalDateItem> = [];

        // Get a collection of date items from all liturgical seasons of the given year
        years.forEach(year => {
            weekdayDates = [
                ...weekdayDates,
                ...Seasons.christmastide(year - 1, christmastideEnds, epiphanyOnJan6, christmastideIncludesTheSeasonOfEpiphany),
                ...Seasons.earlyOrdinaryTime(year, christmastideEnds, epiphanyOnJan6),
                ...Seasons.lent(year),
                ...Seasons.eastertide(year),
                ...Seasons.laterOrdinaryTime(year),
                ...Seasons.advent(year),
                ...Seasons.christmastide(year, christmastideEnds, epiphanyOnJan6, christmastideIncludesTheSeasonOfEpiphany),
            ];

            // Get the celebration dates based on the given year and options
            celebrationsDates = [
                ...celebrationsDates,
                ...Celebrations.dates(year, christmastideEnds, epiphanyOnJan6, corpusChristiOnThursday, ascensionOnSunday),
            ];

            // Get the general calendar based on the given year
            generalDates = [...generalDates, ...Calendar._fetchCalendar("general", year)];

            // Get the relevant national calendar object based on the given year and country
            nationalDates = [...nationalDates, ...Calendar._fetchCalendar(country, year)];
        });

        let sources: Array<Array<IRomcalDateItem>> = [weekdayDates, celebrationsDates, generalDates, nationalDates];

        // Remove all date items not in the given date range
        sources = Calendar._filterItemRange(this.startDate, this.endDate, sources);

        // Remove all date items marked as 'drop' from any other date items
        sources = Calendar._dropItems(sources);

        // Push new item object as a new DateItem
        this._push(sources);

        // Finally, sort the DateItems by date and rank and keep only the relevant
        this._sortAndKeepRelevant();

        return this;
    }

    /**
     * Get all DateItems for a specific calendar
     */
    values(): DateItem[] {
        return this.dateItems;
    }

    /**
     * Get the first DateItem that returns truthy for the predicate object
     */
    _find(predicate: Record<string, Primitive>): DateItem | undefined {
        // eslint-disable-next-line you-dont-need-lodash-underscore/find
        return _.find<DateItem>(this.dateItems, predicate, 0);
    }

    /**
     * Group DateItems by date
     */
    _valuesByDates(): { [key: string]: DateItem[] } {
        return this.dateItems.reduce((result: Record<string, DateItem[]>, value: DateItem) => {
            (result[value.date] = result[value.date] || []).push(value);
            return result;
        }, {});
    }

    /**
     * Remove DateItems that return truthy for the predicate object
     */
    _removeWhere(predicate: Record<string, Primitive>): void {
        _.remove(this.dateItems, predicate);
    }

    /**
     * Push new DateItem objects in the Calendar object
     */
    _push(sources: Array<Array<IRomcalDateItem>>): void {
        sources.forEach((source: Array<IRomcalDateItem>, index: number) =>
            source.forEach((item: IRomcalDateItem) => {
                this._checkAndRemoveExistingItem(item);

                // Get the base DateItem to attach on the new DateItem
                const baseItem = this._find({ date: item.moment.toISOString(), _stack: 0 });

                const { key, name, type, data, ...rest } = item;
                if (key && name && type && data) {
                    // Create a new DateItem and add it to the collection
                    const dateItem = new DateItem({
                        key,
                        name,
                        type,
                        moment: rest.moment,
                        data: data as Required<IRomcalDateItemData>,
                        _stack: index, // Specify the stack index of the source calendar
                        baseItem,
                    });
                    this.dateItems.push(dateItem);
                }
            }),
        );
    }

    /**
     * If a previous date item already exists (has the same key name as the new one),
     * the previous date item will be removed in favour of the new given one,
     * except if the previous item is prioritized but not the new one
     */
    _checkAndRemoveExistingItem(item: IRomcalDateItem): void {
        const previousItems = filter(this.dateItems, "key", item.key);
        if (previousItems.length) {
            previousItems.forEach(previousItem => {
                if (!previousItem.data.prioritized || (previousItem.data.prioritized && item.data && item.data.prioritized)) {
                    this._removeWhere({ _id: previousItem._id });
                }
            });
        }
    }

    /**
     * Sort all DateItems by relevance (the most relevant first)
     * and drop the non-relevant DateItems
     */
    _sortAndKeepRelevant(): void {
        // Reorder the DateItems of a particular day, so that
        // when there are optional memorials or commemoration only (in addition to the feria),
        // the feria item is moved to the top before the optional items,
        // since it's the default item if none of the optional items
        // are celebrated.
        // Sort all date items by relevance (the most relevant first),
        // in this order: by date, by priority, by type, by stack.
        this.dateItems.sort(
            (
                { moment: firstMoment, data: firstData, type: firstType, _stack: firstStack }: DateItem,
                { moment: nextMoment, data: nextData, type: nextType, _stack: nextStack }: DateItem,
            ): number => {
                // 1. Sort by date
                if (firstMoment.isBefore(nextMoment)) {
                    return -1;
                } else if (firstMoment.isAfter(nextMoment)) {
                    return 1;
                } else {
                    // If the date is the same...
                    // 2. Sort by priority (prioritized first)
                    const { prioritized: firstPrioritized } = firstData;
                    const { prioritized: nextPrioritized } = nextData;
                    if (firstPrioritized && !nextPrioritized) {
                        return -1;
                    } else if (!firstPrioritized && nextPrioritized) {
                        return 1;
                    } else {
                        // If neither date is prioritized
                        // 3. Sort by type (higher type first)
                        if (firstType < nextType) {
                            return -1;
                        } else if (firstType > nextType) {
                            return 1;
                        } else {
                            // If the types are the same
                            // 4. Sort by stack (higher stack first)
                            if (firstStack > nextStack) {
                                return -1;
                            } else if (firstStack < nextStack) {
                                return 1;
                            } else {
                                return 0;
                            }
                        }
                    }
                }
            },
        );

        // Now that the items are sorted, let's drop other non-relevant date items
        // if at least one of the date items isn't optional
        const calendarByDates = this._valuesByDates();
        for (const key in calendarByDates) {
            if (Object.prototype.hasOwnProperty.call(calendarByDates, key)) {
                const dateItems = calendarByDates[key];
                if (dateItems.length > 1) {
                    // If the first date item has a type equal or higher than a MEMORIAL, or is prioritized:
                    // keep only the first item and remove the others
                    if (dateItems[0].data.prioritized || dateItems[0].type <= Types.MEMORIAL) {
                        dateItems.slice(1, dateItems.length).forEach(item => this._removeWhere({ _id: item._id }));
                    }
                }
            }
        }
    }

    /**
     * Check if 'drop' has been defined for any celebrations in the national calendar
     * and remove them from both national and general calendar sources
     *
     * @param sources
     */
    static _dropItems(sources: Array<Array<IRomcalDateItem>>): Array<Array<IRomcalDateItem>> {
        // Get an array of keys to be dropped from all sources
        const filteredSources: Array<Array<IRomcalDateItem>> = [
            ...sources.map((source: Array<IRomcalDateItem>) => {
                return source.filter(({ drop }: IRomcalDateItem) => {
                    return isNil(drop) && drop === true;
                });
            }),
        ];
        return filteredSources;
    }

    /**
     * Only retain items within a given date range
     * @param startDate The lower range date to abide by
     * @param endDate The upper range date to abide by
     * @param sources The 2 dimensional array containing various date sources to be filtered
     */
    static _filterItemRange(startDate: Moment, endDate: Moment, sources: Array<Array<IRomcalDateItem>>): Array<Array<IRomcalDateItem>> {
        return sources.map((source: Array<IRomcalDateItem>) =>
            source.filter((item: IRomcalDateItem) => {
                return item.moment.isSameOrAfter(startDate) && item.moment.isSameOrBefore(endDate);
            }),
        );
    }

    /**
     * Get the appropriate calendar definition object, based on the given region name and year.
     *
     * @param country The country to get
     * @param year the year to resolve
     */
    static _fetchCalendar(country: TCountryTypes, year: number): Array<IRomcalDateItem> {
        return CountryCalendars[country].dates(year);
    }
}

function calendarFor<T extends undefined>(): Array<DateItem>;
function calendarFor<T extends IRomcalConfig | number>(
    options?: T,
): T extends number ? Array<DateItem> : "query" extends keyof T ? TRomcalQueryResult<DateItem> | Error : Array<DateItem>;
/**
 * Returns an array of liturgical dates based on the supplied options.
 *
 * The array may return dates according to the given calendar year or liturgical
 * year depending on the options supplied.
 *
 * If a query object is passed, a filtered array of liturgical calendar dates is returned
 * according to the given calendar options and filtering options passed in by the user.
 *
 * If the config object has a query, it will be used to filter the
 * date results returned by an internal call to the `queryFor` method.
 *
 * @param options A configuration object or a year (integer)
 */
function calendarFor(options?: IRomcalConfig | number): Array<DateItem> | TRomcalQueryResult<DateItem> | Error {
    let userConfig: IRomcalConfig = {};

    // If options is passed as an integer,
    // assume we want the calendar for the current year
    // with default options
    if (!isNil(options) && isInteger(options)) {
        userConfig = { year: options };
    }

    // Sanitize incoming config into a complete configuration set
    const config = new Config(userConfig);

    // Set the locale information
    Utils.setLocale(config.locale);

    // Get a new collection of dates
    const dates = new Calendar(config).fetch().values();

    // Determine if there's a query to execute. If none,
    // just return the array if DateItems to the caller
    if (isNil(config.query) || !isObject(config.query) || isNil(options)) {
        return dates as Array<DateItem>;
    } else {
        // Run queries and return the results
        // eslint-disable-next-line @typescript-eslint/no-use-before-define
        return queryFor(dates, config.query) as TRomcalQueryResult<DateItem> | Error;
    }
}

/**
 * Filters an array of dates generated from the calendarFor function based on a given query.
 *
 * @param dates An array of dates generated from the `calendarFor` function
 * @param query A query object containing criteria to filter the dates by
 */
const queryFor = (dates: Array<DateItem>, query: TRomcalQuery = {}): TRomcalQueryResult<DateItem> | Error => {
    if (!dates.every(value => isDateItem(value))) {
        throw new Error("romcal.queryFor can only accept a single dimensional array of DateItem objects");
    }

    //==========================================================================
    // Check if there is a query defined, if none return the unfiltered
    // calendar array
    //==========================================================================
    if (!isNil(query) && isObject(query) && Object.keys(query).length > 0) {
        let filteredResult: TRomcalQueryResult<DateItem> = dates;

        // Months are zero indexed, so January is month 0.
        if (hasKey(query, "month")) {
            filteredResult = dates.filter(dateItem => dateItem.moment.month() === getValueByKey(query, "month"));
        }

        if (hasKey(query, "day")) {
            filteredResult = dates.filter(dateItem => dateItem.moment.day() === getValueByKey(query, "day"));
        }

        if (hasKey(query, "title") && !isNil(query.title)) {
            const { title } = query;
            filteredResult = dates.filter(dateItem => dateItem.data.meta.titles?.includes(title));
        }

        if (hasKey(query, "group")) {
            switch (getValueByKey(query, "group")) {
                case "days":
                    filteredResult = _.groupBy(dates, d => d.moment.day());
                    break;
                case "months":
                    filteredResult = _.groupBy(dates, d => d.moment.month());
                    break;
                case "daysByMonth":
                    // eslint-disable-next-line you-dont-need-lodash-underscore/map
                    filteredResult = _.map(
                        _.groupBy(dates, d => d.moment.month()),
                        monthGroup => _.groupBy(monthGroup, d => d.moment.day()),
                    );
                    break;
                case "weeksByMonth":
                    // eslint-disable-next-line you-dont-need-lodash-underscore/map
                    filteredResult = _.map(
                        _.groupBy(dates, d => d.moment.month()),
                        v => _.groupBy(v, d => d.data.calendar?.week),
                    );
                    break;
                case "cycles":
                    filteredResult = _.groupBy(dates, d => d.data.meta.cycle?.value);
                    break;
                case "types":
                    filteredResult = _.groupBy(dates, d => d.type);
                    break;
                case "liturgicalSeasons":
                    filteredResult = _.groupBy(dates, d => d.data.season.key);
                    break;
                case "liturgicalColors":
                    filteredResult = _.groupBy(dates, d => d.data.meta.liturgicalColor?.key);
                    break;
                case "psalterWeek":
                    filteredResult = _.groupBy(dates, d => d.data.meta.psalterWeek?.key);
                    break;
                default:
                    break;
            }
        }

        return filteredResult;
    }

    return dates;
};

export { calendarFor, queryFor };
