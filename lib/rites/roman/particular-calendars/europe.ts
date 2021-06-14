import { CalendarDef, DateDefinitions } from '../models/calendar-def';
import { Precedences } from '../constants/precedences';
import { LiturgicalColors } from '../constants/colors';

export class Europe extends CalendarDef {
  definitions: DateDefinitions = {
    cyril_the_philosopher_monk_and_methodius_of_thessaloniki_bishop_copatrons_of_europe:
      {
        saints: [
          'cyril_the_philosopher_monk',
          'methodius_of_thessaloniki_bishop',
        ],
        precedence: Precedences.ProperFeast_PrincipalPatronOfARegion_8c,
        date: '2-14',
        liturgicalColors: LiturgicalColors.WHITE,
        titles: (titles) => [...titles, 'Copatron of Europe'],
      },

    catherine_of_siena_virgin: {
      customLocaleKey: 'catherine_of_siena_virgin_copatroness_of_europe',
      precedence: Precedences.ProperFeast_PrincipalPatronOfARegion_8c,
      titles: (titles) => [...titles, 'Copatroness of Europe'],
    },

    benedict_of_nursia_abbot: {
      customLocaleKey: 'benedict_of_nursia_abbot_patron_of_europe',
      precedence: Precedences.ProperFeast_PrincipalPatronOfARegion_8c,
      titles: (titles) => [...titles, 'Patron of Europe'],
    },

    bridget_of_sweden_religious: {
      customLocaleKey: 'bridget_of_sweden_religious_copatroness_of_europe',
      precedence: Precedences.ProperFeast_PrincipalPatronOfARegion_8c,
      titles: (titles) => [...titles, 'Copatroness of Europe'],
    },

    teresa_benedicta_of_the_cross_stein_virgin: {
      customLocaleKey:
        'teresa_benedicta_of_the_cross_stein_virgin_copatroness_of_europe',
      precedence: Precedences.ProperFeast_PrincipalPatronOfARegion_8c,
      titles: (titles) => [...titles, 'Copatroness of Europe'],
    },
  };
}
