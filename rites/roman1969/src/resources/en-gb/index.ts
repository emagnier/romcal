import b from './b.json';
import l from './l.json';
import meta from './meta.json';
import p from './p.json';
import r from './r.json';

export const enGb = {
  ...meta,
  items: {
    ...b.items,
    ...l.items,
    ...p.items,
    ...r.items,
  },
};
