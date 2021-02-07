/**
 * All the possible grouping variants that can be supported by romcal.
 */
export const GROUP_BY = [
  'days',
  'months',
  'daysByMonth',
  'weeksByMonth',
  'sundayCycles',
  'weekdayCycles',
  'ranks',
  'liturgicalSeasons',
  'liturgicalColors',
  'psalterWeeks',
] as const;