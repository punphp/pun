#!/usr/bin/env python3
"""Writes crates/pun-lint/data/ from crates/pun-phpstan/data/symbols.json (PHPStan 2.2.7's reflection of PHP 8.4):
pure-functions.txt, the built-in functions whose hasSideEffects() is No (PureFunctionDetector), and throwables.txt,
the built-in classes that are Throwable or implement it, and bool-functions.txt, the built-in functions every variant
of which returns `bool`, and int-functions.txt, those that return an `int`, lowercase, one a line."""
import json, os, re
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
d = json.load(open(os.path.join(root, 'crates/pun-phpstan/data/symbols.json')))
out = os.path.join(root, 'crates/pun-lint/data')
pure = sorted(k.lower() for k, f in d['functions'].items() if f['sideEffects'] == 'No')
thr = sorted(k.lower() for k, c in d['classes'].items() if c['name'] == 'Throwable' or 'Throwable' in c.get('ancestors', []))
open(os.path.join(out, 'pure-functions.txt'), 'w').write(''.join(n + '\n' for n in pure))
open(os.path.join(out, 'throwables.txt'), 'w').write(''.join(n + '\n' for n in thr))
def is_bool(t):
    # `bool`, or a conditional return such as `($value is array ? true : false)`.
    return t in ('bool', 'true', 'false') or re.fullmatch(r'\(\$\w+ is [^?]+\? (true|false|bool) : (true|false|bool)\)', t) is not None


boolean = sorted(k.lower() for k, f in d['functions'].items() if f['variants'] and all(is_bool(v['return']) for v in f['variants']))
open(os.path.join(out, 'bool-functions.txt'), 'w').write(''.join(n + '\n' for n in boolean))


def is_int(t):
    return re.fullmatch(r'(int|int<[^>]*>|positive-int|negative-int|non-negative-int|non-positive-int|non-zero-int)', t) is not None


integer = sorted(k.lower() for k, f in d['functions'].items() if f['variants'] and all(is_int(v['return']) for v in f['variants']))
open(os.path.join(out, 'int-functions.txt'), 'w').write(''.join(n + '\n' for n in integer))
