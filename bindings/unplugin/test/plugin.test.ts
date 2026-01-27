import { describe, it, expect } from 'vitest';
import { build, type InlineConfig } from 'vite';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import romcalPlugin from '../src/vite.js';
import { generateBundle } from '../src/bundle.js';

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

  describe('different calendars', () => {
    it('should support usa calendar', async () => {
      const code = await buildWithPlugin({
        calendar: 'united_states',
        locale: 'en',
      });

      expect(code).toContain('calendar: "united_states"');
      expect(code).toContain('general_roman');
    });

    it('should support germany calendar with german locale', async () => {
      const code = await buildWithPlugin({
        calendar: 'germany',
        locale: 'de',
      });

      expect(code).toContain('calendar: "germany"');
      expect(code).toContain('locale: "de"');
    });
  });

  describe('configuration options', () => {
    it('should respect epiphanyOnSunday option', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
        epiphanyOnSunday: true,
      });

      // Minified code may use !0 instead of true
      expect(code).toMatch(/epiphany_on_sunday:\s*(!0|true)/);
    });

    it('should respect ascensionOnSunday option', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
        ascensionOnSunday: true,
      });

      // Minified code may use !0 instead of true
      expect(code).toMatch(/ascension_on_sunday:\s*(!0|true)/);
    });

    it('should respect corpusChristiOnSunday option', async () => {
      const code = await buildWithPlugin({
        calendar: 'france',
        locale: 'fr',
        corpusChristiOnSunday: false,
      });

      // Minified code may use !1 instead of false
      expect(code).toMatch(/corpus_christi_on_sunday:\s*(!1|false)/);
    });
  });

  describe('calendar hierarchy', () => {
    it('should include parent calendars in hierarchy', async () => {
      const bundle = await generateBundle({
        calendar: 'france',
        locale: 'fr',
      });

      // France → Europe → General Roman
      const calendarIds = bundle.calendar_definitions.map((c) => c.id);
      expect(calendarIds).toContain('france');
      expect(calendarIds).toContain('europe');
      expect(calendarIds).toContain('general_roman');
    });

    it('should include locale fallback hierarchy', async () => {
      const bundle = await generateBundle({
        calendar: 'france',
        locale: 'fr',
      });

      // fr → en (fallback)
      const locales = bundle.resources.map((r) => r.locale);
      expect(locales).toContain('fr');
      expect(locales).toContain('en');
    });
  });

  describe('error handling', () => {
    it('should throw error for invalid calendar', async () => {
      await expect(
        generateBundle({
          calendar: 'nonexistent_calendar',
          locale: 'en',
        })
      ).rejects.toThrow();
    });

    it('should throw error for invalid locale', async () => {
      await expect(
        generateBundle({
          calendar: 'general_roman',
          locale: 'nonexistent_locale',
        })
      ).rejects.toThrow();
    });

    it('should create bundle even with custom calendar (validation at generate time)', async () => {
      // Custom calendar with a day definition
      // Note: Entity validation happens at calendar generation time, not bundle creation
      const customCalendar = {
        $schema: '../../../../schemas/calendar_definition.json',
        id: 'test_parish',
        metadata: {
          jurisdiction: 'ECCLESIASTICAL',
          type: 'DIOCESE',
        },
        parent_calendar_ids: ['general_roman'],
        days_definitions: {
          // This day references an entity that exists in general_roman resources
          test_day: {
            precedence: 'OPTIONAL_MEMORIAL_12',
            date_def: { month: 1, date: 15 },
          },
        },
      };

      // createBundle() packages data without validating entity references
      // Validation happens when generateLiturgicalCalendar() is called
      const bundle = await generateBundle({
        calendar: 'test_parish',
        locale: 'en',
        calendarDefinitions: [customCalendar as any],
      });

      expect(bundle.calendar).toBe('test_parish');
    });
  });

  describe('custom data', () => {
    it('should merge custom calendar definitions', async () => {
      // Custom calendar definition must follow the schema
      const customCalendar = {
        $schema: '../../../../schemas/calendar_definition.json',
        id: 'my_parish',
        metadata: {
          jurisdiction: 'ECCLESIASTICAL',
          type: 'DIOCESE',
        },
        parent_calendar_ids: ['france'],
        days_definitions: {},
      };

      const bundle = await generateBundle({
        calendar: 'my_parish',
        locale: 'fr',
        calendarDefinitions: [customCalendar as any],
      });

      const calendarIds = bundle.calendar_definitions.map((c) => c.id);
      expect(calendarIds).toContain('my_parish');
      expect(calendarIds).toContain('france');
    });
  });
});
