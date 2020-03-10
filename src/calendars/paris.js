import moment from 'moment';

import { Utils } from '../lib';
import { Titles, Types, LiturgicalColors } from '../constants';

const defaultConfig = {};
const inheritFrom = 'france';

let dates = year => {
  let _dates = [
    {
      key: 'saintGenevieveVirgin',
      type: Types.SOLEMNITY,
      moment: moment.utc({ year: year, month: 0, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      // Same as General, but day + 1
      key: 'theMostHolyNameOfJesus',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 4 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'holyMaryRefugeOfSinner',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 16 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'blessedMarieDeLaProvidence',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintIsabelleOfFranceVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 24 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'blessedDanielBrottierPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintMarieEugenieDeJesusReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintGermainOfParisBishop',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      // Same as France, but Memorial
      key: 'saintClotilde',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 4 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintLandryOfParisBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintLandryOfParisBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'dedicationOfTheCathedralOfParis',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 5, day: 16 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'blessedInnocentVPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'martyrsOfCompiegne',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 17 }),
      meta: {
        liturgicalColor: LiturgicalColors.RED,
        titles: [Titles.MARTYR],
      },
    },
    {
      key: 'saintLouisKingOfFrance',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'blessedMartyrsOfParis',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 2 }),
      meta: {
        liturgicalColor: LiturgicalColors.RED,
        titles: [Titles.MARTYR],
      },
    },
    {
      key: 'saintTeresaOfCalcuttaReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 5 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'blessedFredericOzanam',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 9 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintCeranOfParisBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      // Same as General, but Feast
      key: 'saintDenisAndCompanionsMartyrs',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 9, day: 9 }),
      meta: {
        liturgicalColor: LiturgicalColors.RED,
        titles: [Titles.MARTYR],
      },
    },
    {
      key: 'blessedNicholasBarre',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintMarcelOfParisBishop',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'ourLadyOfTheMiraculousMedal',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 27 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintCatherineLaboureReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
  ];

  // Get localized celebration names
  return Utils.localizeDates(_dates);
};

export { defaultConfig, dates, inheritFrom };
