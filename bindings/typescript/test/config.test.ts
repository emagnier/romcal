import { describe, it, expect } from 'vitest';
import { romcal } from '../src/index.js';

describe('romcal configuration', () => {
  it('should use default configuration', async () => {
    const r = await romcal();

    expect(r.config.calendar).toBe('general_roman');
    expect(r.config.locale).toBe('en');
    expect(r.config.epiphanyOnSunday).toBe(false);
    expect(r.config.corpusChristiOnSunday).toBe(true);
    expect(r.config.ascensionOnSunday).toBe(false);
    expect(r.config.easterCalculationType).toBe('GREGORIAN');
    expect(r.config.context).toBe('GREGORIAN');
  });

  it('should accept calendar and locale as arguments', async () => {
    const r = await romcal('france', 'fr');

    expect(r.config.calendar).toBe('france');
    expect(r.config.locale).toBe('fr');
  });

  it('should accept partial configuration object', async () => {
    const r = await romcal({
      calendar: 'united_states',
      locale: 'en',
      epiphanyOnSunday: true,
      ascensionOnSunday: true,
    });

    expect(r.config.calendar).toBe('united_states');
    expect(r.config.locale).toBe('en');
    expect(r.config.epiphanyOnSunday).toBe(true);
    expect(r.config.ascensionOnSunday).toBe(true);
    // Default values preserved
    expect(r.config.corpusChristiOnSunday).toBe(true);
  });
});
