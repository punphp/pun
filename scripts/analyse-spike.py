#!/usr/bin/env python3
"""Engine spike (context/plan/analyse.md Build 0): which remaining PHPStan level-0 rules Mago's analyser covers.

    cargo build -p punphp-analyse --example mago_fixtures --ignore-rust-version
    target/debug/examples/mago_fixtures > mago-fixtures.jsonl
    scripts/analyse-spike.py table mago-fixtures.jsonl            markdown table on stdout, totals on stderr
    scripts/analyse-spike.py oracle <pinkary copy>                PHPStan + Larastan's types for the Laravel fixture

`oracle` needs a copy of corpus/pinkary with `composer install` done (never the corpus checkout itself). It copies
crates/pun-analyse/tests/fixtures/laravel/app (all but Pinkary's own Link and User and its AppServiceProvider, which
the copy already has) into it, runs `phpstan analyse` on app/Probe.php with the fixture's migrations, and writes the
dumped types to tests/fixtures/laravel/expected.json, for Larastan's parseModelCastsMethod false and true.

Remaining level-0 identifiers: [upstream.phpstan] entries with level "0" in crates/pun-config/rules/mapping.toml that
crates/pun-analyse/src/rules/mod.rs IDENTIFIERS does not list (PHPStan's own phpstanApi/phpParser checks and the
unsupported ones are left out). A `%s` pattern entry stands for the identifiers it matches that have no entry of
their own.

Evidence: PHPStan's own rule fixtures (crates/pun-analyse/tests/fixtures/rules, each case run through
`phpstan analyse --level 0`). A PHPStan error is found when Mago reports, in the same case and file, an issue whose
code [upstream.mago-analyzer] maps to the same Pun rule, on the same line or up to NEARBY lines away, or an issue with
a code from SAME_CHECK. Per identifier: yes = every fixture error found; partial = some found, or no fixture case but a
Mago code maps to the rule; no = otherwise.
"""
import collections, glob, json, os, re, shutil, subprocess, sys, tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RULES = os.path.join(ROOT, 'crates/pun-analyse/tests/fixtures/rules')
# PHPStan reports some checks on another line of the same construct (the first duplicate key, the class).
NEARBY = 10
# Mago codes that make the same check as a PHPStan identifier but that mapping.toml sends to another Pun rule or
# nowhere. Reviewed by hand on the fixture messages (PHPStan's message, then Mago's), e.g.
#   staticProperty.notFound  "Access to an undefined static property static(Bug8034\HelloWorld)::$fields."
#                            "Static property `$fields` does not exist on class `Bug8034\HelloWorld`."
SAME_CHECK = {
    'new.%sConstructor': 'invalid-method-access',
    'property.private': 'non-existent-property',
    'staticProperty.private': 'non-existent-property',
    'property.readWrite': 'incompatible-readonly-modifier',
    'property.visibility': 'incompatible-property-access',
    'assign.invalidExpr': 'invalid-assignment',
    'classConstant.final': 'override-final-constant',
    'method.abstract': 'unimplemented-abstract-method',
    'offsetAccess.noDim': 'array-append-in-read-context',
    'propertySetHook.nativeParameterType': 'incompatible-property-hook-parameter-type',
    'staticProperty.notFound': 'non-existent-property',
    'class.extendsFinalByPhpDoc': 'extend-final-class',
    'class.readOnly': 'invalid-extend',
    'class.nonReadOnly': 'invalid-extend',
    'assign.property%s': 'invalid-property-write',
    'assign.propertyReadOnly': 'invalid-property-write',
    'assign.readOnlyProperty': 'invalid-property-write',
    'assign.readOnlyPropertyByPhpDoc': 'invalid-property-write',
    'method.nonStatic': 'incompatible-static-modifier',
    'method.static': 'incompatible-static-modifier',
    'property.nonStatic': 'incompatible-static-modifier',
    'property.static': 'incompatible-static-modifier',
    'trait.notFound': 'non-existent-class-like',
    'unset.hookedProperty': 'invalid-unset',
    'unset.possiblyHookedProperty': 'invalid-unset',
}


def camel(code):
    return ''.join(p.capitalize() for p in code.split('-'))


def table():
    mapping = tomllib.load(open(os.path.join(ROOT, 'crates/pun-config/rules/mapping.toml'), 'rb'))
    phpstan = mapping['upstream']['phpstan']
    mago = {k: v['pun'] for k, v in mapping['upstream']['mago-analyzer'].items() if 'pun' in v}
    mago_codes = collections.defaultdict(list)
    for code, rule in mago.items():
        mago_codes[rule].append(code)
    src = open(os.path.join(ROOT, 'crates/pun-analyse/src/rules/mod.rs')).read()
    ported = set(re.findall(r'"([^"]+)"', src.split('pub const IDENTIFIERS')[1].split('];')[0]))
    remaining = {k: v for k, v in phpstan.items()
                 if v.get('level') == '0' and 'pun' in v and k not in ported
                 and not k.startswith(('phpstanApi.', 'phpParser.'))}
    patterns = [(re.compile('^' + re.escape(k).replace('%s', '.*') + '$'), k) for k, v in remaining.items()
                if v.get('pattern')]

    def entry(identifier):
        if identifier in remaining and not remaining[identifier].get('pattern'):
            return identifier
        if identifier in phpstan:
            return None
        return next((k for rx, k in patterns if rx.match(identifier)), None)

    want = collections.defaultdict(list)  # entry -> [(case, file, line)]
    for f in glob.glob(os.path.join(RULES, '**/*.json'), recursive=True):
        if '/data/' in f:
            continue
        name = os.path.relpath(f, RULES)[:-5]
        for case in json.load(open(f))['cases']:
            for e in case['errors']:
                key = entry(e.get('identifier') or '')
                if key:
                    want[key].append((name + '::' + case['method'], e['file'], e['line']))
    got = {}
    for line in open(sys.argv[2]):
        r = json.loads(line)
        got[r['test'] + '::' + r['method']] = r['issues']

    rows = collections.defaultdict(list)
    for key, v in sorted(remaining.items()):
        rule = v['pun']
        errors = want.get(key, [])
        exact = nearby = other = 0
        for case, file, line in errors:
            issues = [i for i in got.get(case, []) if i['file'] == file]
            mapped = [i['line'] for i in issues if mago.get(camel(i['code'])) == rule]
            if line in mapped:
                exact += 1
            elif any(abs(m - line) <= NEARBY for m in mapped):
                nearby += 1
            elif any(i['code'] == SAME_CHECK.get(key) and abs(i['line'] - line) <= NEARBY for i in issues):
                other += 1
        found = exact + nearby + other
        codes = mago_codes.get(rule, [])
        if errors:
            verdict = 'yes' if found == len(errors) else 'partial' if found else 'no'
            evidence = f'{found}/{len(errors)} found'
            notes = [f'{nearby} within {NEARBY} lines'] if nearby else []
            if other:
                notes.append(f'{other} as unmapped `{SAME_CHECK[key]}`')
            if notes:
                evidence += ' (' + '; '.join(notes) + ')'
        else:
            verdict = 'partial' if codes else 'no'
            evidence = 'no fixture case; a Mago code maps to the rule' if codes else 'no fixture case'
        rows[rule].append((key, verdict, evidence))

    # Mago issues mapped to a remaining rule where PHPStan reports nothing of that rule nearby: possible extras
    expected = collections.defaultdict(list)
    for key, errors in want.items():
        for case, file, line in errors:
            expected[(case, file, remaining[key]['pun'])].append(line)
    extra = collections.Counter()
    for case, issues in got.items():
        for i in issues:
            rule = mago.get(camel(i['code']))
            if rule in rows and not any(abs(l - i['line']) <= NEARBY for l in expected.get((case, i['file'], rule), [])):
                extra[rule] += 1

    totals = collections.Counter()
    print('| Pun rule | PHPStan identifier | Mago covers | Evidence | Mago codes mapped to the rule |')
    print('| --- | --- | --- | --- | --- |')
    for rule in sorted(rows):
        verdicts = {v for _, v, _ in rows[rule]}
        totals[verdicts.pop() if len(verdicts) == 1 else 'partial'] += 1
        codes = ', '.join(sorted(mago_codes.get(rule, []))) or '-'
        if extra[rule]:
            codes += f' ({extra[rule]} more where PHPStan reports none)'
        for n, (key, verdict, evidence) in enumerate(rows[rule]):
            print(f'| {rule.removeprefix("analyse.") if n == 0 else ""} | `{key}` | {verdict} | {evidence} | '
                  f'{codes if n == 0 else ""} |')
    ids = collections.Counter(v for r in rows.values() for _, v, _ in r)
    print(f'rules: {len(rows)} {dict(totals)}; identifiers: {sum(ids.values())} {dict(ids)}', file=sys.stderr)


FIXTURE = os.path.join(ROOT, 'crates/pun-analyse/tests/fixtures/laravel')
PINKARY_OWN = {'app/Models/Link.php', 'app/Models/User.php', 'app/Providers/AppServiceProvider.php'}


def oracle():
    copy = os.path.abspath(sys.argv[2])
    for f in glob.glob(os.path.join(FIXTURE, 'app/**/*.php'), recursive=True):
        rel = os.path.relpath(f, FIXTURE)
        if rel not in PINKARY_OWN:
            os.makedirs(os.path.dirname(os.path.join(copy, rel)), exist_ok=True)
            shutil.copy(f, os.path.join(copy, rel))
    out = {}
    for parse in ['false', 'true']:
        tmp = os.path.join(copy, 'tmp-probe-' + parse)
        shutil.rmtree(tmp, ignore_errors=True)
        neon = os.path.join(copy, 'probe.neon')
        with open(neon, 'w') as f:
            f.write(f"includes:\n    - vendor/larastan/larastan/extension.neon\nparameters:\n    level: 0\n"
                    f"    tmpDir: {tmp}\n    paths: [app/Probe.php]\n    parseModelCastsMethod: {parse}\n"
                    f"    databaseMigrationsPath: [{os.path.join(FIXTURE, 'database/migrations')}]\n")
        env = {k: v for k, v in os.environ.items() if k not in ('CLAUDECODE', 'CLAUDE_CODE', 'AI_AGENT')}
        r = subprocess.run(['php', 'vendor/phpstan/phpstan/phpstan.phar', 'analyse', '-c', neon, '--error-format=json',
                            '--no-progress'], cwd=copy, env=env, capture_output=True, text=True)
        data = json.loads(r.stdout)
        out['parseModelCastsMethod=' + parse] = sorted(
            [m['line'], m['message'].removeprefix('Dumped type: ')]
            for fe in data['files'].values() for m in fe['messages'] if m['message'].startswith('Dumped type: '))
    with open(os.path.join(FIXTURE, 'expected.json'), 'w') as f:
        json.dump(out, f, indent=1)
        f.write('\n')
    print({k: len(v) for k, v in out.items()})


{'table': table, 'oracle': oracle}[sys.argv[1]]()
