import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { manifest, registry, policy, readJson, requireThat as ok } from './validation.mjs';

const base=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const root=path.resolve(process.argv[2] || base);
policy(base,root,'agents');
const schema=readJson(path.join(root,'schemas/agent.schema.json'));
ok(schema.$schema==='https://json-schema.org/draft/2020-12/schema','unsupported schema draft');


console.log('Validated releases:', registry(root));

console.log('agents: static checks passed (no Agent executed).');
