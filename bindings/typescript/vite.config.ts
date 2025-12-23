import { defineConfig } from 'vite'
import dts from 'vite-plugin-dts'
import { resolve } from 'path'

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
    emptyDirBeforeWrite: true,
    sourcemap: true,
    rollupOptions: {
      external: ['fs', 'path', 'url'],
      output: {
        globals: {
          fs: 'fs',
          path: 'path',
          url: 'url',
        },
      },
    },
  },
})
