import { describe, it, expect } from 'vitest';
import { build, type InlineConfig } from 'vite';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import romcalPlugin from '../src/vite.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

async function buildWithPlugin(
  pluginOptions: Parameters<typeof romcalPlugin>[0] = {},
  entry?: string
): Promise<string> {
  const config: InlineConfig = {
    root: join(__dirname, 'fixtures'),
    configFile: false,
    plugins: [romcalPlugin(pluginOptions)],
    build: {
      write: false,
      lib: {
        entry: entry ?? join(__dirname, 'fixtures/src/main.ts'),
        formats: ['es'],
        fileName: 'main',
      },
      rollupOptions: {
        external: ['romcal'],
      },
    },
    logLevel: 'silent',
  };

  const result = await build(config);

  // build() returns an array when using lib mode
  const outputs = Array.isArray(result) ? result : [result];
  const firstOutput = outputs[0];

  if ('output' in firstOutput && Array.isArray(firstOutput.output)) {
    const chunk = firstOutput.output.find((o) => o.type === 'chunk');
    if (chunk && 'code' in chunk) {
      return chunk.code;
    }
  }

  throw new Error('Could not find code in build output');
}

describe('@romcal/unplugin', () => {
  describe('virtual module resolution', () => {
    it('should resolve virtual:romcal module', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
      });

      expect(code).toContain('france');
      expect(code).toContain('fr');
    });
  });

  describe('bundle content', () => {
    it('should include calendar and locale in bundle', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
      });

      // Minified code uses unquoted property names
      expect(code).toContain('calendar: "france"');
      expect(code).toContain('locale: "fr"');
    });

    it('should include calendar definitions', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
      });

      expect(code).toContain('calendar_definitions');
      expect(code).toContain('general_roman');
    });

    it('should include resources', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
      });

      expect(code).toContain('resources');
      expect(code).toContain('entities');
    });
  });

  describe('default configuration', () => {
    it('should use general_roman and en by default', async () => {
      const code = await buildWithPlugin();

      // Minified code uses unquoted property names
      expect(code).toContain('calendar: "general_roman"');
      expect(code).toContain('locale: "en"');
    });
  });
});
