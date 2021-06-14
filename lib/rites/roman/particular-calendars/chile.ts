import { CalendarDef, DateDefinitions } from '../models/calendar-def';
import { Dates } from '../utils/dates';
import { Precedences } from '../constants/precedences';
import { Dayjs } from 'dayjs';
import { Americas } from './americas';
import { Titles } from '../../../constants/martyrology-metadata';

export class Chile extends CalendarDef {
  inheritFrom = Americas;

  definitions: DateDefinitions = {
    laura_vicuna_virgin: {
      precedence: Precedences.OptionalMemorial_12,
      date: '1-22',
    },

    pius_ix_pope: {
      precedence: Precedences.OptionalMemorial_12,
      date: '2-7',
    },

    our_lady_of_lourdes: {
      precedence: Precedences.ProperMemorial_11b,
      date: '2-11',
    },

    philip_and_james_apostles: {
      date: '5-4',
    },

    teresa_of_jesus_of_los_andes_virgin: {
      precedence: Precedences.ProperFeast_8f,
      date: '7-13',
    },

    camillus_de_lellis_priest: {
      precedence: Precedences.OptionalMemorial_12,
      date: '7-14',
    },

    henry_ii_emperor: {
      precedence: Precedences.OptionalMemorial_12,
      date: '7-14',
    },

    our_lady_of_mount_carmel: {
      customLocaleKey: 'our_lady_of_mount_carmel_mother_and_queen_of_chile',
      precedence: Precedences.ProperSolemnity_PrincipalPatron_4a,
      date: '7-16',
      titles: (titles) => [...titles, Titles.MotherAndQueenOfChile],
    },

    alberto_hurtado_priest: {
      precedence: Precedences.ProperMemorial_11b,
      date: '8-18',
    },

    rose_of_lima_virgin: {
      precedence: Precedences.ProperFeast_8f,
      date: '8-30',
    },

    our_lady_of_mercy: {
      precedence: Precedences.OptionalMemorial_12,
      date: '9-24',
    },

    our_lord_jesus_christ_the_eternal_high_priest: {
      precedence: Precedences.ProperFeast_8f,
      date: (year: number): Dayjs => Dates.pentecostSunday(year).add(4, 'day'),
      // cycles: { liturgicalDayCycle: LiturgicalDayCycle.TEMPORALE },
    },
  };
}
