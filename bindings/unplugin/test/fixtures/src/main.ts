import bundle from 'virtual:romcal';
import { createRomcal } from 'romcal';

async function main() {
  console.log('Bundle calendar:', bundle.calendar);
  console.log('Bundle locale:', bundle.locale);

  const romcal = await createRomcal(bundle);
  const calendar = await romcal.generateLiturgicalCalendar(2026);

  console.log('Easter 2026:', calendar['2026-04-05']);
}

main();
