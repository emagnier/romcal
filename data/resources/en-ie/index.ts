import c from './c.json';
import meta from './meta.json';
import w from './w.json';

export const enIe = {
  ...meta,
  items: {
    ...c.items,
    ...w.items,
  },
};
