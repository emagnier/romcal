import { Colors } from '../constants/colors';
import { Precedences } from '../constants/precedences';
import { CalendarDef } from '../models/calendar-def';
import { Inputs } from '../types/calendar-def';

export class Fmj extends CalendarDef {
  inputs: Inputs = {
    elijah_prophet: {
      precedence: Precedences.ProperMemorial_11b,
      dateDef: { month: 7, date: 20 },
    },

    silouan_the_athonite_monk: {
      precedence: Precedences.OptionalMemorial_12,
      dateDef: { month: 9, date: 24 },
    },

    dismas_the_good_thief: {
      precedence: Precedences.OptionalMemorial_12,
      dateDef: { month: 10, date: 12 },
      colors: Colors.Red,
    },

    charles_of_jesus_de_foucauld: {
      precedence: Precedences.ProperMemorial_11b,
      dateDef: { month: 12, date: 1 },
      colors: Colors.Red,
    },

    david_king: {
      precedence: Precedences.OptionalMemorial_12,
      dateDef: { month: 12, date: 29 },
    },
  };
}
