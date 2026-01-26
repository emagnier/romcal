import { rm, cp } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, '..');

const src = resolve(root, '../docs');
const dest = resolve(root, 'src/content/docs');

await rm(dest, { recursive: true, force: true });
await cp(src, dest, { recursive: true });

console.log('Docs synced successfully');
