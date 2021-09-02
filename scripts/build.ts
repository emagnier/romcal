import chalk from 'chalk';
import { generateDtsBundle } from 'dts-bundle-generator';
import { build, Format, Platform } from 'esbuild';
import fs from 'fs';
import { glob } from 'glob';
import humanizeDuration, { Unit, UnitTranslationOptions } from 'humanize-duration';
import path from 'path';
import rimraf from 'rimraf';
import { PackageJson } from 'type-fest';
import * as ts from 'typescript';
import { GENERAL_ROMAN_NAME } from '../lib';
import { particularCalendars } from '../lib/particular-calendars';
import { toCamelCase } from '../lib/utils/string';
import pkg from '../package.json';
import { RomcalBundler } from './bundle';

const log = console.log;
const tsConfigPath = './tsconfig.release.json';

function reportDiagnostics(diagnostics: ts.Diagnostic[]): void {
  diagnostics.forEach((diagnostic) => {
    let message = 'Error';
    if (diagnostic.file && diagnostic.start) {
      const { line, character } = diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start);
      message += ` ${diagnostic.file.fileName} (${line + 1},${character + 1})`;
    }
    message += ': ' + ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n');
    log(chalk.yellow(message));
  });
}

function readConfigFile(configFileName: string) {
  // Read config file
  const configFileText = fs.readFileSync(configFileName).toString();

  // Parse JSON, after removing comments. Just fancier JSON.parse
  const result = ts.parseConfigFileTextToJson(configFileName, configFileText);
  const configObject = result.config;
  if (!configObject) {
    if (result.error) reportDiagnostics([result.error]);
    process.exit(1);
  }

  // Extract config information
  const configParseResult = ts.parseJsonConfigFileContent(
    configObject,
    ts.sys,
    path.dirname(configFileName),
  );
  if (configParseResult.errors.length > 0) {
    reportDiagnostics(configParseResult.errors);
    process.exit(1);
  }
  return configParseResult;
}

function compile(configFileName: string): void {
  // Extract configuration from config file
  const config = readConfigFile(configFileName);

  // Compile
  const program = ts.createProgram(config.fileNames, config.options);
  const emitResult = program.emit();

  // Report errors
  reportDiagnostics(ts.getPreEmitDiagnostics(program).concat(emitResult.diagnostics));

  // Return code
  const exitCode = emitResult.emitSkipped ? 1 : 0;
  if (exitCode !== 1) process.exit(exitCode);
}

log(chalk.bold(`\n  –– ${chalk.red('Romcal')} builder ––`));
(async () => {
  const time = new Date();

  /**
   * Compiling sources and checking types of the romcal library
   */
  log(chalk.bold(`\n✓ Compiling sources and checking types of the romcal library`));
  rimraf.sync(path.resolve('tmp/dts'));
  compile(tsConfigPath);
  log(chalk.dim(`  .d.ts files created in ./tmp/dts/`));

  /**
   * Create calendar bundle files
   */
  RomcalBundler();

  /**
   * Delete and recreate empty dist directory
   */
  log(chalk.bold(`\n✓ Cleaning up the ${chalk.cyan.bold('./dist/')} directory`));
  rimraf.sync(path.resolve('dist'));
  fs.mkdirSync('dist', { recursive: true });

  /**
   * Bundle all .d.ts files of the core romcal library
   */
  log(
    chalk.bold(
      `\n✓ Bundle all ${chalk.cyan('.d.ts')} files into ${chalk.cyan('./dist/index.d.ts')}`,
    ),
  );
  const dts = generateDtsBundle(
    [
      {
        filePath: './tmp/dts/lib/index.d.ts',
        failOnClass: false,
        libraries: { importedLibraries: ['i18next', 'typescript'] },
        output: {
          sortNodes: true,
          umdModuleName: 'Romcal',
          inlineDeclareExternals: false,
          noBanner: true,
          respectPreserveConstEnum: false,
          exportReferencedTypes: true,
        },
      },
    ],
    { preferredConfigPath: tsConfigPath },
  );
  fs.writeFileSync(path.resolve('dist', 'index.d.ts'), dts.join('\n'), 'utf-8');
  log(chalk.dim(`  ./tmp/dts/lib/index.d.ts → dist/index.d.ts`));

  /**
   * Retrieve all calendar bundles
   */
  const bundles = glob
    .sync('../tmp/bundles/**/*.ts', { cwd: __dirname })
    .map((p) => path.resolve(__dirname, p))
    .filter((p) => !p.match(/\.d\.ts$/));

  const toGlobalName = (calendar: string, locale: string) => {
    const varName = calendar
      .split('.')
      .map((s) => toCamelCase(s))
      .join('_');
    return varName + '_' + toCamelCase(locale);
  };

  /**
   * Build the core library and all calendar bundles
   */
  for (const format of ['cjs', 'esm', 'iife'] as Format[]) {
    log(chalk.bold(`\n✓ Building the codebase using the ${chalk.green.bold(format)} format`));
    const fromFormats: Record<Format, Platform> = { cjs: 'node', esm: 'neutral', iife: 'browser' };
    const platform: Platform = fromFormats[format];

    log(chalk.dim(`  ./lib/index.ts → dist/${format}/romcal.js`));
    await build({
      bundle: true,
      minify: false,
      globalName: 'Romcal',
      sourcemap: 'external',
      ...(format === 'iife' ? {} : { external: ['i18next'] }),
      ...(format === 'iife'
        ? { entryPoints: ['lib/index.iife.ts'] }
        : { entryPoints: ['lib/index.ts'] }),
      format,
      outfile: `dist/${format}/romcal.js`,
    }).catch(() => process.exit(1));

    log(chalk.dim(`  ./tmp/bundles/**/*.ts → dist/bundles/[calendar]/${format}/[locale].js`));
    for (const p of bundles) {
      // Do not output index.ts on iife format
      // and only output locale.iife.js on iife format
      if (
        (format !== 'iife' || !p.match(/\/index\.ts$/)) &&
        ((p.match(/.iife\.ts$/) && format === 'iife') ||
          (!p.match(/.iife\.ts$/) && format !== 'iife'))
      ) {
        const calendar = p.match(/([^/]+)\/[^/]+$/)![1];
        const locale = p.match(/([^/]+)\.\w+$/)![1].replace('.iife', '');
        await build({
          minify: false,
          ...(format === 'iife' ? { globalName: toGlobalName(calendar, locale) } : {}),
          bundle: format === 'iife',
          platform,
          entryPoints: [p],
          format,
          keepNames: true,
          outfile: `dist/bundles/${calendar}/${format}/${locale}.js`,
          sourcemap: false,
          target: 'es2019',
        }).catch(() => process.exit(1));
      }
    }
  }

  /**
   * Add package.json and index.d.ts files to all calendar bundles
   */
  log(chalk.bold(`\n✓ Package calendar bundles as npm modules`));
  const allCalendars = [toCamelCase(GENERAL_ROMAN_NAME), ...Object.keys(particularCalendars)];

  allCalendars.forEach((calendar) => {
    // mixed snake and underscore case to kebab case
    const pkgName = calendar
      .replace(/([A-Z])/g, '-$1')
      .replace(/_/g, '.')
      .toLowerCase();

    const dir = path.resolve(__dirname, `../dist/bundles/${pkgName}`);

    const modulePkg: PackageJson = {
      name: `@romcal/calendar-${pkgName}`,
      version: pkg.version,
      description: `Localized romcal calendar for '${calendar}'`,
      main: 'cjs/index.js',
      module: 'esm/index.js',
      typings: 'index.d.ts',
      engines: pkg.engines,
      repository: pkg.repository,
      keywords: pkg.keywords,
      author: 'The Romcal Team (https://github.com/romcal/romcal)',
      bugs: pkg.bugs,
      homepage: pkg.homepage,
      peerDependencies: { [pkg.name]: pkg.version },
      license: pkg.license,
    };

    fs.writeFileSync(
      path.resolve(dir, 'package.json'),
      JSON.stringify(modulePkg, null, 2),
      'utf-8',
    );

    const currentPath = path.resolve(__dirname, `../tmp/bundles/${pkgName}/index.d.ts`);
    const destinationPath = path.resolve(__dirname, `../dist/bundles/${pkgName}/index.d.ts`);
    fs.copyFileSync(currentPath, destinationPath);
  });
  log(chalk.dim(`  created ${allCalendars.length} modules in ./dist/bundles/`));
  log(chalk.dim(`  created ./package.json to each modules`));
  log(chalk.dim(`  /tmp/bundles/[calendar]/index.d.ts → dist/bundles/[calendar]/index.d.ts`));

  /**
   * Init and display duration helpers
   */
  const units: Unit[] = ['h', 'm', 's'];
  const shortEnglishHumanizer = humanizeDuration.humanizer({
    language: 'shortEn',
    conjunction: ' and ',
    languages: {
      shortEn: units.reduce((acc: UnitTranslationOptions, k) => {
        acc[k] = () => k;
        return acc;
      }, {}),
    },
  });
  const duration = shortEnglishHumanizer(new Date().getTime() - time.getTime(), {
    units,
  }).replace(/(\d+)\s/g, '$1');
  log(chalk.green(`\n✨ Done in ${chalk.bold(duration)}`));
})();