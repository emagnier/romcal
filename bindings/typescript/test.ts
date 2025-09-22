import { romcal } from './src/index.js';

async function testRomcal() {
  console.log('Testing romcal...');

  // Test 1: Default configuration
  const calendar1 = await romcal();

  console.log('calendar1.config.calendar:', calendar1.config.calendar); // 'general_roman'
  console.log('calendar1.config.locale:', calendar1.config.locale); // 'en'
  console.log('calendar1.config.epiphanyOnSunday:', calendar1.config.epiphanyOnSunday); // false
  console.log('calendar1.config.corpusChristiOnSunday:', calendar1.config.corpusChristiOnSunday); // false
  console.log('calendar1.config.ascensionOnSunday:', calendar1.config.ascensionOnSunday); // false
  console.log('calendar1.config.easterCalculationType:', calendar1.config.easterCalculationType); // 'GREGORIAN'

  console.log('calendar1.config:', calendar1.config); // { calendar: 'general_roman', locale: 'en', epiphanyOnSunday: false, ... }

  // Test 2: Custom configuration
  const calendar2 = await romcal('france', 'fr');
  console.log('calendar2.config.calendar:', calendar2.config.calendar); // 'france'
  console.log('calendar2.config.locale:', calendar2.config.locale); // 'fr'

  console.log('All tests passed!');
}

testRomcal().catch(console.error);
