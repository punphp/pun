#!/usr/bin/env python3
"""Writes crates/pun-lint/data/ from crates/pun-phpstan/data/symbols.json (PHPStan 2.2.7's reflection of PHP 8.4):
pure-functions.txt, the built-in functions whose hasSideEffects() is No (PureFunctionDetector), and throwables.txt,
the built-in classes that are Throwable or implement it, lowercase, one a line."""
import json, os
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
d = json.load(open(os.path.join(root, 'crates/pun-phpstan/data/symbols.json')))
out = os.path.join(root, 'crates/pun-lint/data')
pure = sorted(k.lower() for k, f in d['functions'].items() if f['sideEffects'] == 'No')
thr = sorted(k.lower() for k, c in d['classes'].items() if c['name'] == 'Throwable' or 'Throwable' in c.get('ancestors', []))
open(os.path.join(out, 'pure-functions.txt'), 'w').write(''.join(n + '\n' for n in pure))
open(os.path.join(out, 'throwables.txt'), 'w').write(''.join(n + '\n' for n in thr))
