import fs from 'node:fs';
import path from 'node:path';

interface LocaleConfig {
  name: string;
  path: string;
  files: string[];
}

/**
 * Detect automatically all locale directories
 */
function detectLocaleDirectories(resourcesPath: string): string[] {
  return fs
    .readdirSync(resourcesPath, { withFileTypes: true })
    .filter((dirent) => dirent.isDirectory())
    .map((dirent) => dirent.name);
}

/**
 * Detect automatically all JSON files in a locale directory
 */
function detectJsonFiles(localePath: string): string[] {
  return fs
    .readdirSync(localePath)
    .filter((file) => file.endsWith('.json'))
    .map((file) => file.replace('.json', ''))
    .filter((name) => name !== 'meta');
}

/**
 * Format the constant name (xx or xxXx)
 */
function formatConstantName(localeName: string): string {
  // If it's a simple language code (2 characters), keep it as is
  if (localeName.length === 2) {
    return localeName;
  }

  // If it's a language code with region (ex: en-gb), convert to camelCase
  if (localeName.includes('-')) {
    const parts = localeName.split('-');
    return parts[0] + parts[1].charAt(0).toUpperCase() + parts[1].slice(1);
  }

  return localeName;
}

/**
 * Check if a file exists and has identical content
 */
function isFileIdentical(filePath: string, newContent: string): boolean {
  if (!fs.existsSync(filePath)) {
    return false;
  }

  try {
    const existingContent = fs.readFileSync(filePath, 'utf8');
    return existingContent === newContent;
  } catch (error) {
    console.warn(`⚠️ Error reading file ${filePath}:`, error);
    return false;
  }
}

function generateLocaleIndex(config: LocaleConfig): void {
  const { name, path: localePath, files } = config;
  const constantName = formatConstantName(name);

  // Check if the directory exists
  if (!fs.existsSync(localePath)) {
    console.warn(`⚠️ Directory not found: ${localePath}`);
    return;
  }

  // Check if meta.json exists
  if (!fs.existsSync(path.join(localePath, 'meta.json'))) {
    console.warn(`⚠️ Missing meta.json for ${name}`);
  }

  // Check if all files exist
  const missingFiles = files.filter((file) => !fs.existsSync(path.join(localePath, `${file}.json`)));

  if (missingFiles.length > 0) {
    console.warn(`⚠️ Missing files for ${name}:`, missingFiles);
  }

  // Generate all imports (data + meta) - insert meta at its alphabetical position
  const dataFiles = files.filter((file) => fs.existsSync(path.join(localePath, `${file}.json`)));
  const allFiles = [...dataFiles];

  // Insert meta.json at its alphabetical position if it exists
  if (fs.existsSync(path.join(localePath, 'meta.json'))) {
    allFiles.push('meta');
    allFiles.sort();
  }

  const allImports = allFiles
    .map((file) => {
      // Prefix with _ if the name starts with a digit
      const importName = /^\d/.test(file) ? `_${file}` : file;
      return `import ${importName} from './${file}.json';`;
    })
    .join('\n');

  // Generate the spreads for the items
  const spreads = files
    .filter((file) => fs.existsSync(path.join(localePath, `${file}.json`)))
    .map((file) => {
      // Prefix with _ if the name starts with a digit
      const importName = /^\d/.test(file) ? `_${file}` : file;
      return `    ...${importName}.items,`;
    })
    .join('\n');

  const content = `${allImports}

export const ${constantName} = {
  ...meta,
  items: {
${spreads}
  },
};
`;

  const indexPath = path.join(localePath, 'index.ts');

  // Check if file exists and has identical content
  if (isFileIdentical(indexPath, content)) {
    // File exists with identical content, skip logging
    return;
  }

  try {
    fs.writeFileSync(indexPath, content);
    console.log(`✅ Generated index.ts for ${name} (exported as ${constantName})`);
  } catch (error) {
    console.error(`❌ Error writing file ${indexPath}:`, error);
  }
}

/**
 * Generate the main index.ts file at the root of resources
 */
function generateMainIndex(resourcesPath: string, localeDirectories: string[]): void {
  const exports = localeDirectories
    .map((localeName) => {
      const constantName = formatConstantName(localeName);
      return `export { ${constantName} } from './${localeName}';`;
    })
    .join('\n');

  const content = `${exports}
`;

  const mainIndexPath = path.join(resourcesPath, 'index.ts');

  // Check if file exists and has identical content
  if (isFileIdentical(mainIndexPath, content)) {
    // File exists with identical content, skip logging
    return;
  }

  try {
    fs.writeFileSync(mainIndexPath, content);
    console.log(`✅ Generated main index.ts with ${localeDirectories.length} locale exports`);
  } catch (error) {
    console.error(`❌ Error writing file ${mainIndexPath}:`, error);
  }
}

function main(): void {
  const resourcesPath = './src/resources';

  if (!fs.existsSync(resourcesPath)) {
    console.error(`❌ Resources directory not found: ${resourcesPath}`);
    process.exit(1);
  }

  try {
    // Detect automatically all locale directories
    const localeDirectories = detectLocaleDirectories(resourcesPath);

    console.log(`🔍 Found ${localeDirectories.length} locale directories:`, localeDirectories);

    // Generate the index for each locale
    for (const localeName of localeDirectories) {
      const localePath = path.join(resourcesPath, localeName);
      const jsonFiles = detectJsonFiles(localePath);

      const config: LocaleConfig = {
        name: localeName,
        path: localePath,
        files: jsonFiles,
      };

      generateLocaleIndex(config);
    }

    // Generate the main index.ts file at the root of resources
    generateMainIndex(resourcesPath, localeDirectories);

    console.log('✨ All resource index files are up to date!');
  } catch (error) {
    console.error('❌ Error during resource index generation:', error);
    process.exit(1);
  }
}

// Execute the script
main();
