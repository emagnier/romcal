import moment from 'moment';
import _ from 'lodash';

import { Dates, Utils } from '../lib';
import { Titles, Types, LiturgicalColors } from '../constants';

const defaultConfig = {
  christmastideEnds: 'o',
  epiphanyOnJan6: false,
  christmastideIncludesTheSeasonOfEpiphany: true,
  corpusChristiOnThursday: false,
  ascensionOnSunday: false,
};

let dates = year => {
  let _dates = [
    {
      key: 'saintsBasilTheGreatAndGregoryNazianzenBishopsAndDoctors',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 2 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'theMostHolyNameOfJesus',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 3 }),
      data: {},
    },
    {
      key: 'saintRaymondOfPenyafortPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 7 }),
      data: {},
    },
    {
      key: 'saintHilaryOfPoitiersBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 13 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintAnthonyOfEgyptAbbot',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 17 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo split everywhere saintsFabianPopeAndSebastianMartyrs
    {
      key: 'saintsFabianPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 20 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintSebastianMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 20 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAgnesVirginAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintVincentDeaconAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintFrancisDeSalesBishopAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 24 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // The proper color for the Chair of Peter (Feast, Feb 22) and the Conversion of
    //cSt. Paul (Feast, Jan 25) is white, although both St. Peter and St. Paul
    // were martyrs.
    {
      key: 'conversionOfSaintPaulApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 0, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintsTimothyAndTitusBishops',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintAngelaMericiVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 27 }),
      data: {},
    },
    {
      key: 'saintThomasAquinasPriestAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintJohnBoscoPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 0, day: 31 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo split saintBlaseBishopAndMartyrSaintAnsgarBishop everywhere
    {
      key: 'saintBlaseBishopAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAnsgarBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 3 }),
      data: {},
    },
    {
      key: 'saintAgathaVirginAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 5 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintPaulMikiAndCompanionsMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 6 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    // Todo: split saintJeromeEmilianiSaintJosephineBakhitaVirgin everywhere
    {
      key: 'saintJeromeEmilianiPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 8 }),
      data: {},
    },
    {
      key: 'saintJosephineBakhitaVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 8 }),
      data: {},
    },
    {
      key: 'saintScholasticaVirgin',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'ourLadyOfLourdes',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 11 }),
      data: {},
    },
    {
      key: 'saintsCyrilMonkAndMethodiusBishop',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.PATRON_OF_EUROPE],
        },
      },
    },
    {
      key: 'sevenHolyFoundersOfTheServiteOrder',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 17 }),
      data: {},
    },
    {
      key: 'saintPeterDamianBishopAndDoctorOfTheChurch',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 21 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'chairOfSaintPeterApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 1, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintPolycarpBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 1, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCasimir',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 4 }),
      data: {},
    },
    {
      key: 'saintsPerpetuaAndFelicityMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintJohnOfGodReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 8 }),
      data: {},
    },
    {
      key: 'saintFrancesOfRomeReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 9 }),
      data: {},
    },
    {
      key: 'saintPatrickBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 17 }),
      data: {},
    },
    {
      key: 'saintCyrilOfJerusalemBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 18 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintTuribiusOfMogrovejoBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 2, day: 23 }),
      data: {},
    },
    {
      key: 'saintFrancisOfPaolaHermit',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 2 }),
      data: {},
    },
    {
      key: 'saintIsidoreOfSevilleBishopAndDoctorOfTheChurch',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 4 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintVincentFerrerPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 5 }),
      data: {},
    },
    {
      key: 'saintJohnBaptistDeLaSallePriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintStanislausBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 11 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintMartinIPopeAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAnselmOfCanterburyBishopAndDoctorOfTheChurch',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 21 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // Todo: split saintGeorgeMartyrSaintAdalbertBishopAndMartyr everywhere
    {
      key: 'saintGeorgeMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAdalbertBishopAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintFidelisOfSigmaringenPriestAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 24 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintMarkTheEvangelist',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 3, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    // Todo: split saintPeterChanelPriestAndMartyrSaintLouisGrignonDeMontfortPriest separately
    {
      key: 'saintPeterChanelPriestAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintPeterChanelPriestAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCatherineOfSienaVirginAndDoctorOfTheChurch',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 29 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.PATRON_OF_EUROPE, Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintPiusVPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 3, day: 30 }),
      data: {},
    },
    {
      key: 'saintJosephTheWorker',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 1 }),
      data: {},
    },
    {
      key: 'saintAthanasiusBishopAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 2 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintsPhilipAndJamesApostles',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 4, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    // Todo saintsNereusAndAchilleusMartyrsSaintPancrasMartyr everywhere
    {
      key: 'saintsNereusAndAchilleusMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 12 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintPancrasMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 12 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'ourLadyOfFatima',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 13 }),
      data: {},
    },
    {
      key: 'saintMatthiasTheApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 4, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintJohnIPopeAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 18 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintBernardineOfSienaPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 20 }),
      data: {},
    },
    {
      key: 'saintChristopherMagallanesAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintRitaOfCascia',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 22 }),
      data: {},
    },
    // Todo: saintBedeTheVenerablePriestAndDoctorSaintGregoryViiPopeSaintMaryMagdaleneDePazziVirgin
    {
      key: 'saintBedeTheVenerablePriestAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 25 }),
      data: {},
    },
    {
      key: 'saintGregoryViiPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 25 }),
      data: {},
    },
    {
      key: 'saintMaryMagdaleneDePazziVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 25 }),
      data: {},
    },
    {
      key: 'saintPhilipNeriPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintAugustineOfCanterburyBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 27 }),
      data: {},
    },
    // Todo missing popeSaintPaulVI, add it everywhere (in all locales)
    {
      key: 'popeSaintPaulVI',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 4, day: 29 }),
      data: {},
    },
    {
      key: 'visitationOfTheBlessedVirginMary',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 4, day: 31 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJustinMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 1 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintsMarcellinusAndPeterMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 2 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintsCharlesLwangaAndCompanionsMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintBonifaceBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 5 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintNorbertBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 6 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintEphremDeaconAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 9 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintBarnabasTheApostle',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 11 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintAnthonyOfPaduaPriestAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintRomualdAbbot',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 19 }),
      data: {},
    },
    {
      key: 'saintAloysiusGonzagaReligious',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo: split saintPaulinusOfNolaBishopSaintsJohnFisherBishopAndThomasMoreMartyrs everywhere
    {
      key: 'saintPaulinusOfNolaBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 22 }),
      data: {},
    },
    {
      key: 'saintsJohnFisherBishopAndThomasMoreMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCyrilOfAlexandriaBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 27 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintIrenaeusBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'firstMartyrsOfTheChurchOfRome',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 5, day: 30 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintThomasTheApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 6, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintElizabethOfPortugal',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 4 }),
      data: {},
    },
    {
      key: 'saintAnthonyZaccariaPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 5 }),
      data: {},
    },
    {
      key: 'saintMariaGorettiVirginAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 6 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintsAugustineZhaoRongPriestAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 9 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintBenedictOfNursiaAbbot',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 11 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.PATRON_OF_EUROPE],
        },
      },
    },
    {
      key: 'saintHenryBishopAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCamillusDeLellisPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 14 }),
      data: {},
    },
    {
      key: 'saintBonaventureBishopAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 15 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'ourLadyOfMountCarmel',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 16 }),
      data: {},
    },
    {
      key: 'saintApollinaris',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 20 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintLawrenceOfBrindisiPriestAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 21 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // https://github.com/pejulian/romcal/issues/27
    {
      key: 'saintMaryMagdalene',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 6, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // On 11 Feb 2018, Pope Francis inserted the Memorial of the
    // Blessed Virgin Mary, Mother of the Church, into the General Roman Calendar.
    // It will be celebrated on Monday after Pentecost.
    {
      key: 'maryMotherOfTheChurch',
      type: Types.MEMORIAL, // Memorial
      moment: (y => Dates.pentecostSunday(y).add(1, 'days'))(year),
      data: {
        prioritized: true,
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintBridgetOfSwedenReligious',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 6, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.PATRON_OF_EUROPE],
        },
      },
    },
    {
      key: 'saintCharbelMakhloufPriestAndHermit',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 24 }),
      data: {},
    },
    {
      key: 'saintJamesApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 6, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintsJoachimAndAnne',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintMartha',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 29 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintPeterChrysologusBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 30 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintIgnatiusOfLoyolaPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 6, day: 31 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintAlphonsusMariaDeLiguoriBishopAndDoctorOfTheChurch',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 1 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // Todo: split saintEusebiusOfVercelliBishopSaintPeterJulianEymardPriest everywhere
    {
      key: 'saintEusebiusOfVercelliBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 2 }),
      data: {},
    },
    {
      key: 'saintPeterJulianEymardPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 2 }),
      data: {},
    },
    {
      key: 'saintJeanVianneyTheCureOfArsPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 4 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'dedicationOfTheBasilicaOfSaintMaryMajor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 5 }),
      data: {},
    },
    // Todo: Split saintSixtusIiPopeAndCompanionsMartyrsSaintCajetanPriest everywhere
    {
      key: 'saintSixtusIiPopeAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCajetanPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 7 }),
      data: {},
    },
    {
      key: 'saintDominicPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 8 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintTeresaBenedictaOfTheCrossEdithSteinVirginAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 9 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR, Titles.PATRON_OF_EUROPE],
        },
      },
    },
    {
      key: 'saintLawrenceOfRomeDeaconAndMartyr',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 7, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintClareVirgin',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 11 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJaneFrancesDeChantalReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 12 }),
      data: {},
    },
    {
      key: 'saintsPontianPopeAndHippolytusPriestMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintMaximilianMaryKolbePriestAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintStephenOfHungary',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 16 }),
      data: {},
    },
    {
      key: 'saintJohnEudesPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 19 }),
      data: {},
    },
    {
      key: 'saintBernardOfClairvauxAbbotAndDoctorOfTheChurch',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 20 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintPiusXPope',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'queenshipOfBlessedVirginMary',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintRoseOfLima',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 23 }),
      data: {},
    },
    {
      key: 'saintBartholomewTheApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 7, day: 24 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo: split saintLouisSaintJosephOfCalasanzPriest everywhere
    {
      key: 'saintLouisKingOfFrance',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJosephOfCalasanzPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 25 }),
      data: {},
    },
    {
      key: 'saintMonica',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 27 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintAugustineOfHippoBishopAndDoctorOfTheChurch',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'theBeheadingOfSaintJohnTheBaptistMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 7, day: 29 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintGregoryTheGreatPopeAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'birthOfTheBlessedVirginMary',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 8, day: 8 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintPeterClaverPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 9 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'holyNameOfTheBlessedVirginMary',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 12 }),
      data: {},
    },
    {
      key: 'saintJohnChrysostomBishopAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'theExaltationOfTheHolyCross',
      type: Types[4],
      moment: moment.utc({ year: year, month: 8, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'ourLadyOfSorrows',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 15 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintsCorneliusPopeAndCyprianBishopMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 16 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintRobertBellarmineBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 17 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintJanuariusBishopAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 19 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAndrewKimTaegonPriestAndPaulChongHasangAndCompanionsMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 20 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintMatthewApostleAndEvangelist',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 8, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintPioOfPietrelcinaPadrePioPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintsCosmasAndDamianMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintVincentDePaulPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 27 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo split saintWenceslausMartyrSaintsLawrenceRuizAndCompanionsMartyrs everywhere
    {
      key: 'saintWenceslausMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintLawrenceRuizAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintsMichaelGabrielAndRaphaelArchangels',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 8, day: 29 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJeromePriestAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 8, day: 30 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintThereseOfTheChildJesusVirginAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 1 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'guardianAngels',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 2 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintFrancisOfAssisi',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 4 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintBrunoPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 6 }),
      data: {},
    },
    {
      key: 'ourLadyOfTheRosary',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Todo: split saintDenisAndCompanionsMartyrsSaintJohnLeonardiPriest everywhere
    {
      key: 'saintDenisAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 9 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintJohnLeonardiPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 9 }),
      data: {},
    },
    // http://www.vatican.va/roman_curia/congregations/ccdds/documents/rc_con_ccdds_doc_20140529_decreto-calendario-generale-gxxiii-gpii_en.html
    {
      key: 'popeSaintJohnXXIII',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 11 }),
      data: {},
    },
    {
      key: 'saintCallistusIPopeAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintTeresaOfJesusVirginAndDoctorOfTheChurch',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 15 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // Todo: split saintHedwigReligiousSaintMargaretMaryAlacoqueVirgin everywhere
    {
      key: 'saintHedwigReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 16 }),
      data: {},
    },
    {
      key: 'saintMargaretMaryAlacoqueVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 16 }),
      data: {},
    },
    {
      key: 'saintIgnatiusOfAntiochBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 17 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintLukeTheEvangelist',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 9, day: 18 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    // Todo: split saintsJeanDeBrebeufAndIsaacJoguesPriestsAndCompanionsMartyrsSaintPaulOfTheCrossPriest everywhere
    {
      key: 'saintsJeanDeBrebeufAndIsaacJoguesPriestsAndCompanionsMartyrs',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 19 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintPaulOfTheCrossPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 19 }),
      data: {},
    },
    // http://www.vatican.va/roman_curia/congregations/ccdds/documents/rc_con_ccdds_doc_20140529_decreto-calendario-generale-gxxiii-gpii_en.html
    {
      key: 'popeSaintJohnPaulII',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 22 }),
      data: {},
    },
    {
      key: 'saintJohnOfCapistranoPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 23 }),
      data: {},
    },
    {
      key: 'saintAnthonyMaryClaretBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 9, day: 24 }),
      data: {},
    },
    {
      key: 'saintsSimonAndJudeApostles',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 9, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'allSouls',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 10, day: 2 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.PURPLE,
        },
      },
    },
    {
      key: 'saintMartinDePorresReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 3 }),
      data: {},
    },
    {
      key: 'saintCharlesBorromeoBishop',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 4 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    // Replaces 32nd Sunday in Ordinary Time when it falls on a Sunday
    {
      key: 'dedicationOfTheLateranBasilica',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 10, day: 9 }),
      data: {
        prioritized: true,
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintLeoTheGreatPopeAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 10 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintMartinOfToursBishop',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 11 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJosaphatBishopAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 12 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAlbertTheGreatBishopAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 15 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    // Todo: split saintMargaretOfScotlandSaintGertrudeTheGreatVirgin everywhere
    {
      key: 'saintMargaretOfScotland',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 16 }),
      data: {},
    },
    {
      key: 'saintGertrudeTheGreatVirgin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 16 }),
      data: {},
    },
    {
      key: 'saintElizabethOfHungary',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 17 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'dedicationOfTheBasilicasOfSaintsPeterAndPaulApostles',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 18 }),
      data: {},
    },
    {
      key: 'presentationOfTheBlessedVirginMary',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 21 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintCeciliaVirginAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 22 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    // Todo: split saintClementIPopeAndMartyrSaintColumbanReligious everywhere
    {
      key: 'saintClementIPopeAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 23 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintColumbanReligious',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 23 }),
      data: {},
    },
    {
      key: 'saintAndrewDungLacAndCompanionsMartyrs',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 24 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintCatherineOfAlexandriaVirginAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 10, day: 25 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintAndrewTheApostle',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 10, day: 30 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
        },
      },
    },
    {
      key: 'saintFrancisXavierPriest',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 3 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'saintJohnDamascenePriestAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 4 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintNicholasBishop',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 6 }),
      data: {},
    },
    {
      key: 'saintAmbroseBishopAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 7 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintJuanDiegoCuauhtlatoatzin',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 9 }),
      data: {},
    },
    {
      key: 'ourLadyOfLoreto',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 10 }),
      data: {},
    },
    {
      key: 'saintDamasusIPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 11 }),
      data: {},
    },
    {
      key: 'ourLadyOfGuadalupe',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 12 }),
      data: {},
    },
    {
      key: 'saintLucyOfSyracuseVirginAndMartyr',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 13 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintJohnOfTheCrossPriestAndDoctor',
      type: Types.MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 14 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintPeterCanisiusPriestAndDoctor',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 21 }),
      data: {
        meta: {
          titles: [Titles.DOCTOR_OF_THE_CHURCH],
        },
      },
    },
    {
      key: 'saintJohnOfKantyPriest',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 23 }),
      data: {},
    },
    {
      key: 'saintStephenTheFirstMartyr',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 11, day: 26 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintJohnTheApostleAndEvangelist',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 11, day: 27 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.WHITE,
        },
      },
    },
    {
      key: 'holyInnocentsMartyrs',
      type: Types.FEAST,
      moment: moment.utc({ year: year, month: 11, day: 28 }),
      data: {
        meta: {
          liturgicalColor: LiturgicalColors.RED,
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintThomasBecketBishopAndMartyr',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 29 }),
      data: {
        meta: {
          titles: [Titles.MARTYR],
        },
      },
    },
    {
      key: 'saintSylvesterIPope',
      type: Types.OPT_MEMORIAL,
      moment: moment.utc({ year: year, month: 11, day: 31 }),
      data: {},
    },
  ];

  return _.map(_dates, ({ key, data, ...rest }) => {
    let name = Utils.localize({ key: 'sanctoral.' + key });
    let { meta } = data;
    if (_.isUndefined(meta)) {
      data.meta = { titles: [] };
    }
    return { name, key, data, ...rest };
  });
};

export { defaultConfig, dates };
