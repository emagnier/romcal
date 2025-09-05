import { romcal } from './src/index.js';

async function demonstrateRomcal() {
  console.log('=== Demo of Romcal ===\n');

  // Example 1: Default configuration
  console.log('1. Default configuration:');
  const defaultCalendar = await romcal();
  console.log(`   Calendar: ${defaultCalendar.config.calendar}`);
  console.log(`   Locale: ${defaultCalendar.config.locale}`);
  console.log(`   Epiphany on Sunday: ${defaultCalendar.config.epiphanyOnSunday}`);
  console.log(`   Corpus Christi on Sunday: ${defaultCalendar.config.corpusChristiOnSunday}`);
  console.log(`   Ascension on Sunday: ${defaultCalendar.config.ascensionOnSunday}`);
  console.log(`   Type of Easter calculation: ${defaultCalendar.config.easterCalculationType}`);
  console.log();

  // Example 2: Custom configuration
  console.log('2. Custom configuration (France, French):');
  const frenchCalendar = await romcal('france', 'fr');
  console.log(`   Calendar: ${frenchCalendar.config.calendar}`);
  console.log(`   Locale: ${frenchCalendar.config.locale}`);
  console.log();

  // Example 3: Custom configuration (United States, English)
  console.log('3. Custom configuration (United States, English):');
  const usCalendar = await romcal('united_states', 'en');
  console.log(`   Calendar: ${usCalendar.config.calendar}`);
  console.log(`   Locale: ${usCalendar.config.locale}`);
  console.log();

  // Example 4: Object configuration (calendar3 from PLAN_TO_ACTION.md)
  console.log('4. Object configuration (France Paris, French, custom settings):');
  const parisCalendar = await romcal({
    calendar: 'france__paris',
    locale: 'fr',
    epiphanyOnSunday: true,
    corpusChristiOnSunday: true,
    ascensionOnSunday: true,
    easterCalculationType: 'JULIAN',
  });
  console.log(`   Calendar: ${parisCalendar.config.calendar}`);
  console.log(`   Locale: ${parisCalendar.config.locale}`);
  console.log(`   Epiphany on Sunday: ${parisCalendar.config.epiphanyOnSunday}`);
  console.log(`   Corpus Christi on Sunday: ${parisCalendar.config.corpusChristiOnSunday}`);
  console.log(`   Ascension on Sunday: ${parisCalendar.config.ascensionOnSunday}`);
  console.log(`   Type of Easter calculation: ${parisCalendar.config.easterCalculationType}`);
  console.log();

  // Example 5: Partial configuration (only some options specified)
  console.log('5. Partial configuration (only calendar and some boolean options):');
  const partialCalendar = await romcal({
    calendar: 'france',
    epiphanyOnSunday: true,
    easterCalculationType: 'JULIAN',
  });
  console.log(`   Calendar: ${partialCalendar.config.calendar}`);
  console.log(`   Locale: ${partialCalendar.config.locale} (default)`);
  console.log(`   Epiphany on Sunday: ${partialCalendar.config.epiphanyOnSunday}`);
  console.log(`   Corpus Christi on Sunday: ${partialCalendar.config.corpusChristiOnSunday} (default)`);
  console.log(`   Ascension on Sunday: ${partialCalendar.config.ascensionOnSunday} (default)`);
  console.log(`   Type of Easter calculation: ${partialCalendar.config.easterCalculationType}`);
  console.log();

  // Example 6: Complete configuration (JSON)
  console.log('6. Complete configuration (JSON):');
  console.log(JSON.stringify(defaultCalendar.config, null, 2));
}

demonstrateRomcal().catch(console.error);
