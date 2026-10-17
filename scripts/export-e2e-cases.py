#!/usr/bin/env python3
"""Write crates/pun-cli/tests/e2e/rules.json, the manifest of crates/pun-cli/tests/rules_e2e.rs: one small project
per rule, with what real Pint does to it.

Rules: every fixer Pinkary's config enables (docs/presets/pinkary-effective.json, with its effective options), plus
new_with_parentheses and Pint/phpdoc_type_annotations_only (laravel preset / Pest config), plus the 17 Pun/ rules.
For a Pint rule the input is the first upstream fixture input (crates/pun-harness/fixtures/<rule>.json) that real
Pint changes with `{"preset": "empty", "rules": {<rule>: <options>}}` (falling back to the fixture case's own
configuration); `expected` is the file Pint writes and `test_stdout`/`test_exit` what `pint --test` prints and
returns. Pun/ rules are enabled through pun.toml; their input/expected is the first Mago-derived case of the
fixer's own tests (crates/pun-fixers/src/fixers/pun_*.rs `mago_cases`), there being no Pint to ask.

usage: scripts/export-e2e-cases.py   (needs php 8.5 and .oracle/pint; ~3 minutes)
"""
import json, os, shutil, subprocess, tempfile, tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PINT = os.path.join(ROOT, '.oracle/pint/builds/pint')
OUT = os.path.join(ROOT, 'crates/pun-cli/tests/e2e/rules.json')
MAPPING = tomllib.load(open(os.path.join(ROOT, 'crates/pun-config/rules/mapping.toml'), 'rb'))
NODE_MODULES = os.path.join(ROOT, 'crates/pun-fixers/node_modules')  # prettier + prettier-plugin-blade
ENV = {'PATH': os.environ['PATH'], 'HOME': os.environ.get('HOME', '/tmp')}

# First mago_cases pair of each Pun/ fixer (crates/pun-fixers/src/fixers/pun_<name>.rs), wrappers expanded.
PUN = {
    'Pun/final_controller': ("<?php\n\nclass UserController\n{\n}\n", "<?php\n\nfinal class UserController\n{\n}\n"),
    'Pun/no_implicit_model_query': ("<?php\n\n$users = User::where('is_active', true)->get();\n", "<?php\n\n$users = User::query()->where('is_active', true)->get();\n"),
    'Pun/no_only': ("<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n})->only();\n", "<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n});\n"),
    'Pun/no_redundant_isset': ("<?php isset($a[0], $a[0][1]);", "<?php isset($a[0][1]);"),
    'Pun/no_redundant_literal_return': ("<?php\n\nfunction getValue($x) {\n    if ($x === null) {\n        return null;\n    }\n    return $x;\n}\n", "<?php\n\nfunction getValue($x) {\n    return $x;\n}\n"),
    'Pun/no_redundant_nullsafe': ("<?php\n\n$name = $user?->name ?? 'Guest';\n\nif (isset($user?->profile)) {\n    // Do something with $user->profile\n}\n", "<?php\n\n$name = $user->name ?? 'Guest';\n\nif (isset($user->profile)) {\n    // Do something with $user->profile\n}\n"),
    'Pun/no_self_assignment': ("<?php\n\n$a = $a;\n$this->x = $this->x;\n$foo->bar = $foo->bar;\n", "<?php\n\n$this->x = $this->x;\n$foo->bar = $foo->bar;\n"),
    'Pun/prefer_array_spread': ("<?php\n\n$merged = array_merge($a, $b, $c);\n", "<?php\n\n$merged = [...$a, ...$b, ...$c];\n"),
    'Pun/prefer_array_validation_rules': (
        "<?php\n\nclass StoreUserRequest\n{\n    public function rules(): array\n    {\n        return [\n            'name' => 'required|string|max:255',\n        ];\n    }\n}\n",
        "<?php\n\nclass StoreUserRequest\n{\n    public function rules(): array\n    {\n        return [\n            'name' => ['required', 'string', 'max:255'],\n        ];\n    }\n}\n"),
    'Pun/prefer_dedicated_status_assertion': ("<?php\n\n$response->assertStatus(404);\n", "<?php\n\n$response->assertNotFound();\n"),
    'Pun/prefer_explode_over_preg_split': ("<?php\n\n$parts = preg_split('/, /', $s);\n", "<?php\n\n$parts = explode(', ', $s);\n"),
    'Pun/prefer_fake_helper': (
        "<?php\n\nclass UserFactory\n{\n    public function definition(): array\n    {\n        return ['name' => $this->faker->unique()->word()];\n    }\n}\n",
        "<?php\n\nclass UserFactory\n{\n    public function definition(): array\n    {\n        return ['name' => fake()->unique()->word()];\n    }\n}\n"),
    'Pun/prefer_first_class_callable': ("<?php\n\nrun(fn($x) => $this->method($x));\n", "<?php\n\nrun($this->method(...));\n"),
    'Pun/sensitive_parameter': ("<?php\n\nfunction login(string $username, string $password): void {\n    // ...\n}\n", "<?php\n\nfunction login(string $username, #[\\SensitiveParameter] string $password): void {\n    // ...\n}\n"),
    'Pun/use_dedicated_expectation': ("<?php\n\nexpect(is_array($value))->toBeTrue();\n", "<?php\n\nexpect($value)->toBeArray();\n"),
    'Pun/use_simpler_expectation': ("<?php\n\nexpect($a > $b)->toBeTrue();\n", "<?php\n\nexpect($a)->toBeGreaterThan($b);\n"),
    'Pun/use_specific_expectations': ("<?php\n\nexpect($value)->toBe(true);\n", "<?php\n\nexpect($value)->toBeTrue();\n"),
}


def file_for(rule):
    return 'test.blade.php' if rule == 'Pint/laravel_blade' else 'test.php'


def pint(rule, options, code, test):
    d = tempfile.mkdtemp()
    f = file_for(rule)
    try:
        json.dump({'preset': 'empty', 'rules': {rule: options}}, open(os.path.join(d, 'pint.json'), 'w'))
        if rule == 'Pint/laravel_blade':
            os.symlink(NODE_MODULES, os.path.join(d, 'node_modules'))
        open(os.path.join(d, f), 'w', newline='').write(code)
        r = subprocess.run(['php', PINT] + (['--test'] if test else []), cwd=d, env=ENV, capture_output=True, text=True, timeout=120)
        return r.returncode, r.stdout, open(os.path.join(d, f), newline='').read()
    finally:
        shutil.rmtree(d)


# Rules whose upstream inputs need a file (BOM, Blade): hand inputs, tried first.
HAND = {
    'encoding': '\ufeff<?php\necho 1;\n',
    'Pint/laravel_blade': open(os.path.join(ROOT, 'crates/pun-fixers/tests/blade/comments/blade-multiline.blade.php')).read(),
}


def fixture_cases(rule):
    """Upstream testFix inputs (default whitespace config, no file), then pun-fixers' oracle cases."""
    if rule in HAND:
        yield HAND[rule], None
    path = os.path.join(ROOT, 'crates/pun-harness/fixtures', rule.replace('/', '_') + '.json')
    for test in json.load(open(path))['tests'] if os.path.exists(path) else []:
        names = [p['name'] for p in test['params']]
        if not test['method'].startswith('testFix') or 'input' not in names:
            continue
        for case in test.get('cases', []):
            args = case.get('args', [])
            at = lambda n: args[names.index(n)] if n in names and names.index(n) < len(args) else None
            if isinstance(at('input'), str) and not at('whitespacesConfig') and not at('file'):
                config = at('configuration') or at('config')
                yield at('input'), config if isinstance(config, dict) and config else None
    path = os.path.join(ROOT, 'crates/pun-fixers/tests/cases', rule.replace('/', '_') + '.json')
    for case in json.load(open(path)) if os.path.exists(path) else []:
        if 'input' in case and 'path' not in case:
            yield case['input'], case.get('config') or None


def main():
    effective = json.load(open(os.path.join(ROOT, 'docs/presets/pinkary-effective.json')))
    rules = {f['name']: (f['options'] if f['options'] else True) for f in effective['fixers']}
    rules.setdefault('new_with_parentheses', True)
    rules.setdefault('Pint/phpdoc_type_annotations_only', True)
    rules.update({c['name']: True for c in effective['custom_fixers']})
    cases, missing = [], []
    for rule, options in sorted(rules.items()):
        found = None
        for pass_ in ('effective', 'case'):
            for code, case_config in fixture_cases(rule):
                opts = options if pass_ == 'effective' else case_config
                if opts is None:
                    continue
                _, _, fixed = pint(rule, opts, code, False)
                if fixed != code:
                    found = (code, opts, fixed)
                    break
            if found:
                break
        if not found:
            # Nothing PHP 8.5 accepts makes it fire (e.g. clean_namespace, normalize_index_brace): still pin that
            # Pun leaves the first fixture input alone too.
            missing.append(rule)
            code = next(fixture_cases(rule), (None, None))[0]
            if code is None:
                continue
            found = (code, options, pint(rule, options, code, False)[2])
        code, opts, fixed = found
        exit_code, stdout, _ = pint(rule, opts, code, True)
        cases.append({'rule': rule, 'file': file_for(rule), 'pint.json': {'preset': 'empty', 'rules': {rule: opts}}, 'input': code, 'expected': fixed,
                      'test_stdout': stdout, 'test_exit': exit_code})
        print(f'{rule}: ok', flush=True)
    for rule, (code, expected) in PUN.items():
        name = next(v['pun'] for k, v in MAPPING['upstream']['pun'].items() if k.replace('-', '_') == rule).split('.', 1)[1]
        cases.append({'rule': rule, 'file': 'test.php', 'pun.toml': f'[format]\npreset = "empty"\n\n[lint.rules]\n{name} = {{ fix = "apply" }}\n', 'input': code,
                      'expected': expected, 'test_exit': 1})
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(cases, open(OUT, 'w'), indent=1, ensure_ascii=False)
    print(f'wrote {OUT}: {len(cases)} cases; not firing: {missing}')


main()
