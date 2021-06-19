import { CalendarDef, DateDefinitions } from '../models/calendar-def';
import { Precedences } from '../constants/precedences';
import { Europe } from './europe';

export class Croatia extends CalendarDef {
  inheritFrom = Europe;

  definitions: DateDefinitions = {
    aloysius_stepinac_bishop: {
      precedence: Precedences.ProperMemorial_11b,
      date: '2-10',
    },

    ivan_merz: {
      precedence: Precedences.ProperMemorial_11b,
      date: '5-10',
    },

    leopold_mandic_priest: {
      precedence: Precedences.ProperMemorial_11b,
      date: '5-12',
    },

    quirinus_of_sescia_bishop: {
      precedence: Precedences.OptionalMemorial_12,
      date: '6-4',
    },

    mary_of_jesus_crucified_petkovic_virgin: {
      precedence: Precedences.OptionalMemorial_12,
      date: '7-9',
    },

    our_lady_of_marija_bistrica: {
      precedence: Precedences.ProperFeast_8f,
      date: '7-13',
    },

    augustine_kazotic_bishop: {
      precedence: Precedences.OptionalMemorial_12,
      date: '8-3',
    },

    marko_krizin_priest: {
      precedence: Precedences.ProperMemorial_11b,
      date: '9-7',
    },

    nicholas_tavelic_priest: {
      precedence: Precedences.ProperFeast_8f,
      date: '11-14',
    },
  };
}