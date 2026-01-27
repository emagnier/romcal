import { defineConfig } from 'vite';
import romcal from '../../src/vite.js';

export default defineConfig({
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
  build: {
    outDir: 'dist',
    lib: {
      entry: 'src/main.ts',
      formats: ['es'],
      fileName: 'main',
    },
    minify: false,
  },
});
