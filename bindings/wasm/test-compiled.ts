// Test file that will be compiled to JavaScript
import { romcal } from './src/index.js';

async function testRomcal() {
  console.log('Testing romcal (compiled version)...');

  // Test 1: Default configuration
  const calendar1 = await romcal();

  console.log('calendar1.config.calendar:', calendar1.config.calendar);
  console.log('calendar1.config.locale:', calendar1.config.locale);
  console.log('calendar1.config.epiphanyOnSunday:', calendar1.config.epiphanyOnSunday);
  console.log('calendar1.config.corpusChristiOnSunday:', calendar1.config.corpusChristiOnSunday);
  console.log('calendar1.config.ascensionOnSunday:', calendar1.config.ascensionOnSunday);
  console.log('calendar1.config.easterCalculationType:', calendar1.config.easterCalculationType);

  // Test 2: Custom configuration
  const calendar2 = await romcal('france', 'fr');
  console.log('calendar2.config.calendar:', calendar2.config.calendar);
  console.log('calendar2.config.locale:', calendar2.config.locale);

  console.log('All tests passed!');
}

testRomcal().catch(console.error);
