import dayjs from "dayjs";

import { Locales } from "../lib";
import { Titles, Types, LiturgicalColors } from "../constants";
import { IRomcalDateItem } from "../models/romcal-date-item";
import { IRomcalDefaultConfig } from "../models/romcal-config";

const defaultConfig: IRomcalDefaultConfig | undefined = undefined;

const dates = async (year: number): Promise<Array<IRomcalDateItem>> => {
    const _dates: Array<IRomcalDateItem> = [
        {
            key: "saintPubliusBishop",
            type: Types.MEMORIAL,
            moment: dayjs.utc(`${year}-1-22`),
        },
        {
            key: "shipwreckOfSaintPaulApostle",
            type: Types.SOLEMNITY,
            moment: dayjs.utc(`${year}-2-10`),
        },
        {
            key: "saintsCyrilMonkAndMethodiusBishop",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-2-14`),
            data: {
                meta: {
                    liturgicalColor: LiturgicalColors.WHITE,
                    titles: [Titles.PATRON_OF_EUROPE],
                },
            },
        },
        {
            key: "blessedMariaAdeodataPisaniVirgin",
            type: Types.OPT_MEMORIAL,
            moment: dayjs.utc(`${year}-2-25`),
        },
        {
            key: "ourLadyOfSorrows",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-4-15`),
        },
        {
            key: "saintCatherineOfSienaVirginAndDoctorOfTheChurch",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-4-29`),
            data: {
                meta: {
                    titles: [Titles.DOCTOR_OF_THE_CHURCH, Titles.PATRON_OF_EUROPE],
                },
            },
        },
        {
            key: "saintPiusVPope",
            type: Types.MEMORIAL,
            moment: dayjs.utc(`${year}-4-30`),
        },
        {
            key: "saintGeorgePrecaPriest",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-5-9`),
        },
        {
            key: "blessedNazjuFalzon",
            type: Types.OPT_MEMORIAL,
            moment: dayjs.utc(`${year}-7-1`),
        },
        {
            key: "saintBenedictOfNursiaAbbot",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-7-11`),
        },
        {
            key: "ourLadyOfMountCarmel",
            type: Types.MEMORIAL,
            moment: dayjs.utc(`${year}-7-16`),
        },
        {
            key: "saintBridgetOfSwedenReligious",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-7-23`),
            data: {
                meta: {
                    liturgicalColor: LiturgicalColors.WHITE,
                    titles: [Titles.PATRON_OF_EUROPE],
                },
            },
        },
        {
            key: "saintTeresaBenedictaOfTheCrossEdithSteinVirginAndMartyr",
            type: Types.FEAST,
            moment: dayjs.utc(`${year}-8-9`),
            data: {
                meta: {
                    liturgicalColor: LiturgicalColors.RED,
                    titles: [Titles.MARTYR, Titles.PATRON_OF_EUROPE],
                },
            },
        },
        {
            key: "saintCatherineOfAlexandriaVirginAndMartyr",
            type: Types.MEMORIAL,
            moment: dayjs.utc(`${year}-11-25`),
            data: {
                meta: {
                    liturgicalColor: LiturgicalColors.RED,
                    titles: [Titles.MARTYR],
                },
            },
        },
    ];

    // Get localized celebration names
    return await Locales.localizeDates(_dates);
};

export { dates, defaultConfig };
