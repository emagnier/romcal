import { Precedences } from '../constants/precedences';
import { CalendarDef } from '../models/calendar-def';
import { Inputs } from '../types/calendar-def';

import { France_Coutances } from './france.coutances';

export class France_MontSaintMichel extends CalendarDef {
  ParentCalendar = France_Coutances;

  inputs: Inputs = {
    dedication_of_the_basilica_of_mont_saint_michel_france: {
      precedence: Precedences.ProperSolemnity_DedicationOfTheOwnChurch_4b,
    },

    dedication_of_consecrated_churches_on_october_25: {
      drop: true,
    },

    dedication_of_consecrated_churches_on_last_sunday_of_october: {
      drop: true,
    },
  };
}
