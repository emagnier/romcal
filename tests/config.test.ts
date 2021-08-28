import 'jest-extended';
import { CalendarScopes } from '../lib/constants/calendar-scopes';
import { RomcalConfig } from '../lib/models/config';
import { LiturgicalDayConfig } from '../lib/models/liturgical-day-config';

describe('getConfig()', () => {
  test('should get general config if country doesnt have default configurations', async () => {
    const config = new RomcalConfig();
    const liturgicalDayConfig = new LiturgicalDayConfig(config);
    const { year, scope, epiphanyOnSunday, corpusChristiOnSunday, ascensionOnSunday } =
      liturgicalDayConfig.getConfigObject();

    expect(year).toBe(new Date().getFullYear());
    expect(scope).toBe(CalendarScopes.Gregorian);
    expect(epiphanyOnSunday).toBeTrue();
    expect(corpusChristiOnSunday).toBeTrue();
    expect(ascensionOnSunday).toBeFalse();
  });
});
