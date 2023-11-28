import assert from 'assert';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import stripAnsi from 'strip-ansi';

assert(
  ['linux', 'darwin'].includes(process.platform),
  'This DEMO only runs on linux or macOS.'
);

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const m = { exports: {} };
process.dlopen(
  m,
  path.join(
    __dirname,
    `target/${process.env.R ? 'release' : 'debug'}/libpackage.dylib`
  )
);

m.exports.error();
// m.exports.panicAsyncBacktrace();
