import { Colors } from '../constants/colors';
import { Precedences } from '../constants/precedences';
import { CalendarDef } from '../models/calendar-def';
import { Inputs } from '../types/calendar-def';

import { France_Paris } from './france.paris';

export class France_SaintGervais extends CalendarDef {
  parentCalendar = France_Paris;

  inputs: Inputs = {
    // our_lady_refuge_of_sinners: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    // louise_de_marillac_religious: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    // mary_of_the_incarnation_barbara_acarie_religious: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    // landry_of_paris_bishop: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    gervais_and_protais_martyrs: {
      // Specific to Paris
      precedence: Precedences.ProperFeast_ToAnIndividualChurch_8e,
      dateDef: { month: 6, date: 19 },
      colors: Colors.Red,
    },

    // carmelites_of_compiegne_virgins_and_martyrs: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    dedication_of_the_church_of_saint_gervais: {
      // Specific to Paris
      precedence: Precedences.ProperSolemnity_DedicationOfTheOwnChurch_4b,
      dateDef: { month: 10, date: 25 },
    },

    dedication_of_consecrated_churches_on_october_25: {
      drop: true,
    },

    dedication_of_consecrated_churches_on_last_sunday_of_october: {
      drop: true,
    },

    // our_lady_of_the_miraculous_medal: {
    //   precedence: Precedences.ProperMemorial_11b,
    // },

    // catherine_zoe_laboure_virgin: {
    //   precedence: Precedences.ProperMemorial_11b,
    // }
  };
}
