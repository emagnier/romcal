import { defineConfig } from 'vite'
import dts from 'vite-plugin-dts'
import { resolve } from 'node:path'

export default defineConfig({
  plugins: [
    dts({
      include: ['src/**/*.ts'],
      exclude: ['src/**/*.test.ts', 'test.ts'],
      rollupTypes: true,
    }),
  ],
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'Romcal',
      formats: ['es', 'umd'],
      fileName: (format) => `romcal.${format === 'es' ? 'js' : 'umd.js'}`,
    },
    outDir: 'dist',
    emptyOutDir: true,
    sourcemap: true,
    rollupOptions: {
      external: ['node:fs', 'node:path', 'node:url'],
      output: {
        globals: {
          'node:fs': 'fs',
          'node:path': 'path',
          'node:url': 'url',
        },
      },
    },
  },
})
